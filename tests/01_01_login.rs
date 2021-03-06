mod helper;

#[test]
fn t01_login_with_valid_id_secret() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("login")
        .arg("-c")
        .arg(helper::CLIENT_ID)
        .arg("-s")
        .arg(helper::CLIENT_SECRET)
        .output();

    assert_eq!(
        helper::TestHelper::parse_stdout(output)?,
        "Login successful."
    );
    Ok(())
}

#[test]
fn t02_login_and_verify() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command().arg("test").output();
    let result = helper::TestHelper::parse_stdout(output)?;

    assert!(result.starts_with("Connecting ... Ok: GET: Pong"));
    assert!(result.ends_with(", admin"));

    Ok(())
}
