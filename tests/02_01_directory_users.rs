mod helper;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    pub avatar: Option<String>,
    pub display_name: String,
    pub id: String,
    pub name: String,
    pub url: String,
}

#[test]
fn t01_directory_users_list() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command().arg("directory").arg("users").arg("list").output();

    let result = helper::TestHelper::parse_list_stdout_to_json::<User>(output)?;

    result
        .values
        .iter()
        .find(|x| {
            x.name == "13146781153"
                && x.display_name == "shaunxu"
                && x.id == "1a28a1241b3644439c93f8d464d5170e"
                && x.url == "https://open.worktile.com/v1/directory/users/1a28a1241b3644439c93f8d464d5170e"
        })
        .unwrap();
    result
        .values
        .iter()
        .find(|x| {
            x.name == "shaunxu1"
                && x.display_name == "shaunxu1"
                && x.id == "6be9cd1b050e4afc932d00b7e8db2961"
                && x.url == "https://open.worktile.com/v1/directory/users/6be9cd1b050e4afc932d00b7e8db2961"
        })
        .unwrap();
    result
        .values
        .iter()
        .find(|x| {
            x.name == "shaunxu2"
                && x.display_name == "shaunxu2"
                && x.id == "d55f16b4480945c29d12482c331c5aa3"
                && x.url == "https://open.worktile.com/v1/directory/users/d55f16b4480945c29d12482c331c5aa3"
        })
        .unwrap();
    result
        .values
        .iter()
        .find(|x| {
            x.name == "shaunxu3"
                && x.display_name == "shaunxu3"
                && x.id == "a9d62798bccf4c18b29fcd0e4ad2cd83"
                && x.url == "https://open.worktile.com/v1/directory/users/a9d62798bccf4c18b29fcd0e4ad2cd83"
        })
        .unwrap();
    result
        .values
        .iter()
        .find(|x| {
            x.name == "shaunxu4"
                && x.display_name == "shaunxu4"
                && x.id == "1307a7f29e5e4ca78a78a6584a307bd9"
                && x.url == "https://open.worktile.com/v1/directory/users/1307a7f29e5e4ca78a78a6584a307bd9"
        })
        .unwrap();

    Ok(())
}

#[test]
fn t02_directory_users_get() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("directory")
        .arg("users")
        .arg("get")
        .arg("--id")
        .arg("6be9cd1b050e4afc932d00b7e8db2961")
        .output();

    let result = helper::TestHelper::parse_stdout_to_json::<User>(output)?;

    assert_eq!(result.name, "shaunxu1");
    assert_eq!(result.display_name, "shaunxu1");
    assert_eq!(result.id, "6be9cd1b050e4afc932d00b7e8db2961");
    assert_eq!(result.url, "https://open.worktile.com/v1/directory/users/6be9cd1b050e4afc932d00b7e8db2961");

    Ok(())
}
