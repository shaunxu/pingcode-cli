mod helper;

#[test]
fn t01_exec() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command().output();

    assert_eq!(helper::TestHelper::parse_stdout(output)?, "");
    Ok(())
}
