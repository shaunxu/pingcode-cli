extern crate reqwest;

use crate::wt_error;
use crate::AnyError;

use serde::Deserialize;
use serde::Serialize;
use std::io::Seek;

#[derive(Debug, Serialize, Deserialize)]
struct ReleaseAsset {
    url: String,
    id: i32,
    name: String,
    content_type: String,
    size: i32,
    browser_download_url: String,
}

impl ReleaseAsset {
    pub async fn perform_update(&self, bin_name: String) -> Result<(), AnyError> {
        // create a temporary folder for asset download
        let tmp_dir = tempfile::Builder::new().prefix("pc_update_").tempdir_in(std::env::current_dir()?)?;
        let tmp_dir_path = tmp_dir.path();
        println!("Preparing temporary folder for update at {}", tmp_dir_path.to_str().unwrap());
        let tmp_tarball_path = tmp_dir_path.join(self.name.clone());
        // let tmp_dir_path = std::path::Path::new("./");
        // let tmp_tarball_path = tmp_dir_path.join("./pc-darwin-x86.tar.gz");
        let mut tmp_tarball = std::fs::OpenOptions::new().read(true).write(true).create(true).open(&tmp_tarball_path)?;
        // download asset
        println!("Downloading new version from {}", &self.url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", crate_name!().parse()?);
        headers.insert("Accept", "application/octet-stream".parse()?);
        let req = reqwest::Client::new().request(reqwest::Method::GET, &self.url);
        let res = req.headers(headers).send().await?;
        if res.status().is_success() {
            let bytes = res.bytes().await?;
            let mut slice: &[u8] = bytes.as_ref();
            std::io::copy(&mut slice, &mut tmp_tarball)?;
            // extract tarball into binary
            println!("Extract release asset to binary from {} to {}", self.name, bin_name);
            tmp_tarball.seek(std::io::SeekFrom::Start(0))?;
            let decompressed = flate2::read::GzDecoder::new(tmp_tarball);
            let mut archive = tar::Archive::new(decompressed);
            let entry = archive
                .entries()
                .unwrap()
                .find(|x| x.as_ref().unwrap().header().path().unwrap().to_str().unwrap() == bin_name);
            if let Some(entry) = entry {
                entry?.unpack_in(tmp_dir_path)?;
                // replace binary
                let src = tmp_dir_path.join(bin_name.clone());
                let dest = &std::env::current_exe()?;
                let tmp = tmp_dir_path.join("replacement_tmp");
                if dest.exists() {
                    std::fs::rename(dest, &tmp)?;
                    if let Err(e) = std::fs::rename(src, dest) {
                        std::fs::rename(&tmp, dest)?;
                        Err(Box::new(e))
                    } else {
                        println!("Update success, please run \"{} --version\" to verify.", bin_name.clone());
                        Ok(())
                    }
                } else {
                    std::fs::rename(src, dest)?;
                    println!("Update success, please run \"{} --version\" to verify.", bin_name.clone());
                    Ok(())
                }
            } else {
                Err(wt_error::WTError::new_boxed(
                    "000000",
                    &format!("Cannot find entry \"{}\" from tarball \"{}\"", bin_name, tmp_tarball_path.to_str().unwrap()),
                ))
            }
        } else {
            Err(wt_error::WTError::new_boxed("000000", &format!("Download asset failed with status code {}", res.status())))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Release {
    url: String,
    assets_url: String,
    id: i32,
    tag_name: String,
    draft: bool,
    prerelease: bool,
    assets: std::vec::Vec<ReleaseAsset>,
}

impl Release {
    pub async fn latest() -> Result<Self, AnyError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", crate_name!().parse()?);
        headers.insert("content-type", "application/json".parse()?);
        let req = reqwest::Client::new().request(reqwest::Method::GET, "https://api.github.com/repos/shaunxu/pingcode-cli/releases/latest");
        let res = req.headers(headers).send().await?;
        if res.status().is_success() {
            let release: Release = res.json().await?;
            Ok(release)
        } else {
            Err(wt_error::WTError::new_boxed(
                "000000",
                &format!("Download latest release metadata failed with status code {}", res.status()),
            ))
        }
    }

    pub fn get_update_asset(&self, asset_name: String) -> Result<Option<&ReleaseAsset>, AnyError> {
        let current_version = semver::Version::parse(crate_version!())?;
        let latest_version = semver::Version::parse(&self.tag_name[1..])?;
        if latest_version > current_version {
            for asset in self.assets.iter() {
                if asset.name == asset_name {
                    return Ok(Some(asset));
                }
            }
            Ok(None)
        } else {
            Ok(None)
        }
    }
}

pub struct Updater {}

impl Updater {
    pub async fn update(dry_run: bool) -> Result<(), AnyError> {
        if let Some((platform, bin_name)) = match std::env::consts::OS {
            "linux" => Some(("linux", "pc")),
            "macos" => Some(("darwin", "pc")),
            "windows" => Some(("win", "pc.exe")),
            _ => None,
        } {
            let release = Release::latest().await?;
            if let Some(asset) = release.get_update_asset(format!("pc-{}-x64.tar.gz", platform))? {
                println!("New version {} avavilable. (Your are running v{})", release.tag_name, crate_version!());
                if !dry_run {
                    asset.perform_update(String::from(bin_name)).await
                } else {
                    Ok(())
                }
            } else {
                println!("Your are running the latest version v{}, enjoy.", crate_version!());
                Ok(())
            }
        } else {
            Err(wt_error::WTError::new_boxed("000000", "Unknown operation system"))
        }
    }
}
