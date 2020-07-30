mod helper;

use helper::TestHelper;
use helper::TestResult;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct State {
    pub color: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub url: String,
}

#[test]
fn t01_agile_states_list() -> TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("states")
        .arg("list")
        .arg("--work-item-type")
        .arg("story")
        .arg("--project-type")
        .arg("scrum")
        .output();

    let result = TestHelper::parse_list_stdout_to_json::<State>(output)?;

    result
        .values
        .iter()
        .find(|x| x.id == "5f005eec5dc73c0017c389da" && x.name == "打开" && x.r#type == "pending")
        .unwrap();

    result
        .values
        .iter()
        .find(|x| x.id == "5f005eec5dc73c0017c389db" && x.name == "进行中" && x.r#type == "in_progress")
        .unwrap();

    Ok(())
}

#[test]
fn t02_agile_states_get() -> TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("states")
        .arg("get")
        .arg("--id")
        .arg("5f005eec5dc73c0017c389db")
        .output();

    let result = TestHelper::parse_stdout_to_json::<State>(output)?;

    assert_eq!(result.id, "5f005eec5dc73c0017c389db");
    assert_eq!(result.name, "进行中");
    assert_eq!(result.r#type, "in_progress");

    Ok(())
}
