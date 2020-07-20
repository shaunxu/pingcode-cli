pub type AnyError = Box<dyn std::error::Error>;
pub type TestResult = Result<(), AnyError>;

pub const CLIENT_ID: &'static str = "gfPSNzkhLfSq";
pub const CLIENT_SECRET: &'static str = "ySsDAYxpaZKSztuTDZGWHMAr";

pub struct TestHelper {}
impl TestHelper {
    pub fn get_exe_command() -> std::process::Command {
        test_bin::get_test_bin("pc")
    }

    pub fn parse_output(pipe: std::vec::Vec<u8>) -> String {
        String::from(String::from_utf8_lossy(&pipe).to_string().trim_end_matches('\n'))
    }

    pub fn parse_stdout(output: std::io::Result<std::process::Output>) -> Result<String, AnyError> {
        Ok(TestHelper::parse_output(output?.stdout))
    }
}