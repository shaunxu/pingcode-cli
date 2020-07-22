mod helper;

#[test]
fn run_with_no_command_or_arg() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command().output();

    assert_eq!(helper::TestHelper::parse_stdout(output)?, "");
    Ok(())
}
