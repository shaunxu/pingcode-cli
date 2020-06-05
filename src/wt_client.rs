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
}

impl std::fmt::Display for WTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code = \"{}\", message = \"{}\"", self.code, self.message)
    }
}

impl std::error::Error for WTError {}

pub struct WTClient {
    client: Client,
    api_endpoint: String,
    client_id: String,
    client_secret: String,
    access_token: String,
    expires_in: i32,
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
            access_token: String::default(),
            expires_in: 0,
        }
    }

    async fn request(
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
        }
        else if res.status().is_client_error() {
            let err: WTError = serde_json::from_value(res.json().await?)?;
            Err(Box::new(err))
        }
        else {
            Err(Box::new(WTError::new("000000", "Unknown Server Error")))
        }

        // let res = req.send().await?;
        // if res.status().is_success() {
        //     Ok(ClientResponse::Success(res.json().await?))
        // } else if res.status().is_client_error() {
        //     Ok(ClientResponse::ClientError(res.json().await?))
        // } else {
        //     Ok(ClientResponse::UnknownError())
        // }
        // match res {
        //     Ok(res) => Ok(ClientResponse::Success(res.json().await?)),
        //     Err(e) => Err(Box::new(e)),
        // }

        // let status_code = res.status();
        // if status_code.is_success() {
        //     Ok(ClientResponse::Success(res.json().await?))
        // }
        // else if status_code.is_client_error() {
        //     Ok(ClientResponse::ClientError(res.json().await?))
        // }
        // else {
        //     Err(std::error::Error::)
        // }

        // Ok(req.send().await?.json().await?)
    }

    pub async fn auth(&mut self) -> Result<(), AnyError> {
        let uri = format!(
            "v1/auth/token?grant_type=client_credentials&client_id={}&client_secret={}",
            self.client_id, self.client_secret
        );
        self.access_token = String::default();

        // let res = self.request(Method::GET, &uri, None, None).await?;
        // match res {
        //     ClientResponse::Success(res) => {
        //         let json: AuthResponse = serde_json::from_value(res)?;
        //         self.access_token = json.access_token;
        //         self.expires_in = json.expires_in;
        //         Ok(())
        //     }
        // }

        let res: AuthResponse =
            serde_json::from_value(self.request(Method::GET, &uri, None, None).await?)?;
        self.access_token = res.access_token;
        self.expires_in = res.expires_in;
        Ok(())
    }
}
