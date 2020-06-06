use super::AnyError;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct WTError {
    code: String,
    message: String,
}

impl WTError {
    pub fn new(code: &str, message: &str) -> WTError {
        WTError {
            code: String::from(code),
            message: String::from(message),
        }
    }

    pub fn new_boxed(code: &str, message: &str) -> Box<WTError> {
        let err = WTError::new(code, message);
        Box::new(err)
    }
}

impl std::fmt::Display for WTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "code = \"{}\", message = \"{}\"",
            self.code, self.message
        )
    }
}

impl std::error::Error for WTError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct WTClientHost {
    api_endpoint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WTClientCredentials {
    client_id: Option<String>,
    client_secret: Option<String>,
    access_token: Option<String>,
    expires_in: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WTConfig {
    host: Option<WTClientHost>,
    credentials: Option<WTClientCredentials>,
}

impl WTConfig {
    pub fn load(path: String) -> WTConfig {
        if let Ok(content) = std::fs::read_to_string(path) {
            let config: WTConfig = toml::from_str(&content).unwrap();
            config
        } else {
            WTConfig {
                host: Some(WTClientHost {
                    api_endpoint: Some(String::from("https://open.worktile.com")),
                }),
                credentials: None,
            }
        }
    }

    pub fn save(&self, path: &String) -> Result<(), AnyError> {
        let content = toml::to_string_pretty(self).unwrap();
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        write!(file, "{}", content)?;
        Ok(())
    }
}

pub struct WTClient {
    client: Client,
    config_path: String,
    config: WTConfig,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

impl WTClient {
    pub fn new(config_path: Option<String>) -> WTClient {
        let path = match config_path {
            Some(p) => p,
            None => String::from(".wt_config"),
        };
        WTClient {
            client: Client::new(),
            config_path: path.clone(),
            config: WTConfig::load(path.clone()),
        }
    }

    async fn request_internal(
        &self,
        method: Method,
        api_endpoint: &str,
        uri: &str,
        body: Option<&serde_json::Value>,
        headers: Option<HeaderMap>,
    ) -> Result<serde_json::Value, AnyError> {
        let url = format!("{}/{}", api_endpoint, uri);
        let mut req = self.client.request(method, &url);
        req = match headers {
            Some(headers) => req.headers(headers),
            None => req,
        };
        req = match body {
            Some(body) => req.json(body),
            None => req,
        };

        let res = req.send().await?;
        if res.status().is_success() {
            Ok(res.json().await?)
        } else if res.status().is_client_error() {
            let err: WTError = serde_json::from_value(res.json().await?)?;
            Err(Box::new(err))
        } else {
            Err(WTError::new_boxed("000000", "Unknown Server Error"))
        }
    }

    async fn request(
        &mut self,
        method: Method,
        uri: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AnyError> {
        let api_endpoint = self
            .config
            .host
            .as_ref()
            .and(self.config.host.as_ref().unwrap().api_endpoint.clone());
        let credentials = self.config.credentials.as_ref();
        let client_id = credentials.and(credentials.unwrap().client_id.clone());
        let client_secret = credentials.and(credentials.unwrap().client_secret.clone());
        let access_token = credentials.and(credentials.unwrap().access_token.clone());
        if access_token.is_none() {
            if client_id.is_none() || client_secret.is_none() || api_endpoint.is_none() {
                return Err(WTError::new_boxed("000000", "Please login first"));
            } else {
                self.auth(
                    &client_id.unwrap(),
                    &client_secret.unwrap(),
                    &api_endpoint.clone().unwrap(),
                )
                .await?;
            }
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            format!(
                "Bearer {}",
                self.config
                    .credentials
                    .as_ref()
                    .unwrap()
                    .access_token
                    .as_ref()
                    .unwrap()
            )
            .parse()
            .unwrap(),
        );
        self.request_internal(method, &api_endpoint.unwrap(), uri, body, Some(headers))
            .await
    }

    pub async fn auth(
        &mut self,
        client_id: &String,
        client_secret: &String,
        api_endpoint: &String,
    ) -> Result<(), AnyError> {
        let uri = format!(
            "v1/auth/token?grant_type=client_credentials&client_id={}&client_secret={}",
            client_id, client_secret
        );
        let res: AuthResponse = serde_json::from_value(
            self.request_internal(Method::GET, &api_endpoint, &uri, None, None)
                .await?,
        )?;
        self.config.host = Some(WTClientHost {
            api_endpoint: Some(api_endpoint.clone()),
        });
        self.config.credentials = Some(WTClientCredentials {
            client_id: Some(client_id.clone()),
            client_secret: Some(client_secret.clone()),
            access_token: Some(res.access_token),
            expires_in: Some(res.expires_in),
        });
        self.config.save(&self.config_path)?;
        Ok(())
    }

    pub async fn ping(&mut self) -> Result<String, AnyError> {
        let res = self.request(Method::GET, "v1/auth/ping", None).await?;
        let res: serde_json::Value = serde_json::from_value(res)?;
        if let serde_json::Value::String(pong) = &res["data"] {
            Ok(pong.clone())
        } else {
            Err(WTError::new_boxed("000000", "Invalid ping response"))
        }
    }
}
