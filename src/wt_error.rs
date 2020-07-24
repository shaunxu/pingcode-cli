use serde::Deserialize;
use serde::Serialize;

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
        Box::new(WTError::new(code, message))
    }
}

impl std::fmt::Display for WTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code = \"{}\", message = \"{}\"", self.code, self.message)
    }
}

impl std::error::Error for WTError {}
