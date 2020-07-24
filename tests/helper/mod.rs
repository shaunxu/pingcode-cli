#![allow(dead_code)]

use serde::Deserialize;

pub type AnyError = Box<dyn std::error::Error>;
pub type TestResult = Result<(), AnyError>;

pub const CLIENT_ID: &'static str = "gfPSNzkhLfSq";
pub const CLIENT_SECRET: &'static str = "ySsDAYxpaZKSztuTDZGWHMAr";

pub struct UIDs<'a> {
    pub shaunxu: &'a str,
    pub shaunxu1: &'a str,
    pub shaunxu2: &'a str,
    pub shaunxu3: &'a str,
    pub shaunxu4: &'a str,
}

pub const UIDS: UIDs = UIDs {
    shaunxu: "1a28a1241b3644439c93f8d464d5170e",
    shaunxu1: "6be9cd1b050e4afc932d00b7e8db2961",
    shaunxu2: "d55f16b4480945c29d12482c331c5aa3",
    shaunxu3: "a9d62798bccf4c18b29fcd0e4ad2cd83",
    shaunxu4: "1307a7f29e5e4ca78a78a6584a307bd9",
};

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

    pub fn to_body_string<T: serde::Serialize>(body: T) -> Result<String, AnyError> {
        Ok(serde_json::to_string(&body)?)
    }
}
