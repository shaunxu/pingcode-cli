#![allow(dead_code)]

use serde::Deserialize;

pub type AnyError = Box<dyn std::error::Error>;
pub type TestResult = Result<(), AnyError>;

pub const CLIENT_ID: &'static str = "gfPSNzkhLfSq";
pub const CLIENT_SECRET: &'static str = "ySsDAYxpaZKSztuTDZGWHMAr";

#[derive(Debug, Deserialize)]
pub struct ApiListResult<T> {
    pub page_index: i32,
    pub page_size: i32,
    pub total: i32,
    pub values: std::vec::Vec<T>,
}

pub struct TestHelper {}
impl TestHelper {
    pub fn get_exe_command() -> std::process::Command {
        test_bin::get_test_bin("pc")
    }

    pub fn parse_output(pipe: std::vec::Vec<u8>) -> String {
        String::from(
            String::from_utf8_lossy(&pipe)
                .to_string()
                .trim_end_matches('\n'),
        )
    }

    pub fn parse_stdout(output: std::io::Result<std::process::Output>) -> Result<String, AnyError> {
        Ok(TestHelper::parse_output(output?.stdout))
    }

    pub fn parse_stdout_to_json<T: serde::de::DeserializeOwned>(
        output: std::io::Result<std::process::Output>,
    ) -> Result<T, AnyError> {
        let content = TestHelper::parse_stdout(output)?;
        let json: serde_json::Value = serde_json::from_str(&content)?;
        match serde_json::from_value::<T>(json) {
            Ok(json) => Ok(json),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn parse_list_stdout_to_json<T: serde::de::DeserializeOwned>(
        output: std::io::Result<std::process::Output>,
    ) -> Result<ApiListResult<T>, AnyError> {
        TestHelper::parse_stdout_to_json::<ApiListResult<T>>(output)
    }
}
