use crate::wt_error;
use crate::AnyError;

use serde::Deserialize;
use serde::Serialize;

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
        println!("{}", bin_name);
        println!("{:#?}", self);
        Ok(())
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
        let mut req = reqwest::Client::new().request(
            reqwest::Method::GET,
            "https://api.github.com/repos/shaunxu/pingcode-cli/releases/latest",
        );
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", crate_name!().parse()?);
        headers.insert("content-type", "application/json".parse().unwrap());
        req = req.headers(headers);
        let res = req.send().await?;
        if res.status().is_success() {
            let release: Release = res.json().await?;
            Ok(release)
        } else {
            Err(wt_error::WTError::new_boxed(
                "000000",
                &format!("Download failed with status code {}", res.status()),
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
                println!(
                    "New version {} avavilable. (Your are running v{}.)",
                    release.tag_name,
                    crate_version!()
                );
                if !dry_run {
                    asset.perform_update(String::from(bin_name)).await
                } else {
                    Ok(())
                }
            } else {
                println!(
                    "Your are running the latest version v{}, enjoy.",
                    crate_version!()
                );
                Ok(())
            }

            // if release.need_update()? {
            //     println!(
            //         "New version {} avavilable. (Your are running v{}.)",
            //         release.tag_name,
            //         crate_version!()
            //     );
            //     if dry_run {
            //         Ok(())
            //     } else {
            //         release.perform_update(format!("pc-{}-x64.tar.gz", platform)).await
            //     }
            // } else {
            //     println!(
            //         "Your are running the latest version v{}, enjoy.)",
            //         crate_version!()
            //     );
            //     Ok(())
            // }

        // let mut req = reqwest::Client::new().request(
        //     reqwest::Method::GET,
        //     "https://api.github.com/repos/shaunxu/pingcode-cli/releases/latest",
        // );
        // let mut headers = reqwest::header::HeaderMap::new();
        // headers.insert("User-Agent", crate_name!().parse()?);
        // headers.insert("content-type", "application/json".parse().unwrap());
        // req = req.headers(headers);
        // let res = req.send().await?;
        // if res.status().is_success() {
        //     let release: Release = res.json().await?;
        // if let Some(asset) = release.assets.into_iter().find(|x| x.name == asset_name) {
        //     let tmp_tarball_path = format!("./{}", asset.name);
        //     let tmp_tarball = std::fs::OpenOptions::new()
        //         .read(true)
        //         .write(true)
        //         .create(true)
        //         .open(&tmp_tarball_path)?;
        //     Updater::download_asset(asset.browser_download_url, tmp_tarball).await?;

        //     Ok(())
        // } else {
        //     Err(wt_error::WTError::new_boxed(
        //         "000000",
        //         &format!("Cannot find asset by name {}", asset_name),
        //     ))
        // }
        // } else {
        //     Err(wt_error::WTError::new_boxed(
        //         "000000",
        //         &format!("Download failed with status code {}", res.status()),
        //     ))
        // }
        } else {
            Err(wt_error::WTError::new_boxed(
                "000000",
                "Unknown operation system",
            ))
        }
    }
}
