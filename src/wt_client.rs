use super::AnyError;
use reqwest::header::HeaderMap;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

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

pub struct WTClient {
    client: Client,
    api_endpoint: String,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    expires_in: Option<i32>,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    access_token: String,
    expires_in: i32,
    token_type: String,
}

impl WTClient {
    pub fn new(api_endpoint: String, client_id: String, client_secret: String) -> WTClient {
        WTClient {
            client: Client::new(),
            api_endpoint: api_endpoint,
            client_id: client_id,
            client_secret: client_secret,
            access_token: None,
            expires_in: None,
        }
    }

    async fn request_internal(
        &self,
        method: Method,
        uri: &str,
        body: Option<&serde_json::Value>,
        headers: Option<HeaderMap>,
    ) -> Result<serde_json::Value, AnyError> {
        let url = format!("{}/{}", self.api_endpoint, uri);
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
        body: Option<&serde_json::Value>
    ) -> Result<serde_json::Value, AnyError> {
        if let None = self.access_token {
            self.auth().await?;
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            format!("Bearer {}", self.access_token.as_ref().unwrap()).parse().unwrap(),
        );
        self.request_internal(method, uri, body, Some(headers)).await
    }

    pub async fn auth(&mut self) -> Result<(), AnyError> {
        let uri = format!(
            "v1/auth/token?grant_type=client_credentials&client_id={}&client_secret={}",
            self.client_id, self.client_secret
        );
        let res: AuthResponse =
            serde_json::from_value(self.request_internal(Method::GET, &uri, None, None).await?)?;
        self.access_token = Some(res.access_token);
        self.expires_in = Some(res.expires_in);

        Ok(())
    }

    pub async fn ping(&mut self) -> Result<String, AnyError> {
        let res = self.request(Method::GET, "v1/auth/ping", None).await?;
        let res: serde_json::Value = serde_json::from_value(res)?;
        if let serde_json::Value::String(pong) = &res["data"] {
            Ok(pong.clone())
        }
        else {
            Err(WTError::new_boxed("000000", "Invalid response"))
        }
    }
}
