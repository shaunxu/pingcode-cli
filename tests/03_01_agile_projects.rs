mod helper;

use serde::Deserialize;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::vec::Vec;
use helper::TestHelper;

#[derive(Debug, Deserialize)]
struct ProjectMember {
    pub avatar: Option<String>,
    pub display_name: String,
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    pub color: String,
    pub description: Option<String>,
    pub id: String,
    pub identifier: String,
    pub name: String,
    pub r#type: String,
    pub url: String,
    pub members: Vec<ProjectMember>,
}

#[derive(Debug)]
struct Fixture {
    pub project_scrum_id: Option<String>,
    pub project_kanban_id: Option<String>,
}

static FIXTURE: Lazy<Mutex<Fixture>> = Lazy::new(|| Mutex::new(Fixture {
    project_scrum_id: None,
    project_kanban_id: None,
}));

#[test]
fn t01_agile_projects_list() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("projects")
        .arg("list")
        .output();

    let result = TestHelper::parse_list_stdout_to_json::<Project>(output)?;

    let project_scrum = result.values.iter().find(|x| {
        x.id == "5f0952e3ee4c4a6f47bf696c"
            && x.identifier == "YIG"
            && x.r#type == "scrum"
            && x.url == "https://open.worktile.com/v1/agile/projects/5f0952e3ee4c4a6f47bf696c"
    }).unwrap();
    FIXTURE.lock().unwrap().project_scrum_id = Some(project_scrum.id.clone());
    project_scrum.members.iter().find(|x| x.name == "13146781153").unwrap();
    project_scrum.members.iter().find(|x| x.name == "shaunxu1").unwrap();

    let project_kanban = result.values.iter().find(|x| {
        x.id == "5f0952fa3012ac4f71f65b18"
            && x.identifier == "YIK"
            && x.r#type == "kanban"
            && x.url == "https://open.worktile.com/v1/agile/projects/5f0952fa3012ac4f71f65b18"
    }).unwrap();
    FIXTURE.lock().unwrap().project_kanban_id = Some(project_kanban.id.clone());
    project_kanban.members.iter().find(|x| x.name == "13146781153").unwrap();
    project_kanban.members.iter().find(|x| x.name == "shaunxu2").unwrap();

    Ok(())
}

#[test]
fn t02_agile_projects_list_scrum() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("projects")
        .arg("list")
        .arg("--type")
        .arg("scrum")
        .output();

    let result = TestHelper::parse_list_stdout_to_json::<Project>(output)?;

    let project_scrum = result.values.iter().find(|x| {
        x.id == "5f0952e3ee4c4a6f47bf696c"
            && x.identifier == "YIG"
            && x.r#type == "scrum"
            && x.url == "https://open.worktile.com/v1/agile/projects/5f0952e3ee4c4a6f47bf696c"
    }).unwrap();
    project_scrum.members.iter().find(|x| x.name == "13146781153").unwrap();
    project_scrum.members.iter().find(|x| x.name == "shaunxu1").unwrap();

    Ok(())
}

#[test]
fn t03_agile_projects_list_kanban() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("projects")
        .arg("list")
        .arg("--type")
        .arg("kanban")
        .output();

    let result = TestHelper::parse_list_stdout_to_json::<Project>(output)?;

    let project_kanban = result.values.iter().find(|x| {
        x.id == "5f0952fa3012ac4f71f65b18"
            && x.identifier == "YIK"
            && x.r#type == "kanban"
            && x.url == "https://open.worktile.com/v1/agile/projects/5f0952fa3012ac4f71f65b18"
    }).unwrap();
    project_kanban.members.iter().find(|x| x.name == "13146781153").unwrap();
    project_kanban.members.iter().find(|x| x.name == "shaunxu2").unwrap();

    Ok(())
}

#[test]
fn t04_agile_projects_get() -> helper::TestResult {
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("projects")
        .arg("get")
        .arg("--id")
        .arg(FIXTURE.lock().unwrap().project_scrum_id.as_ref().unwrap())
        .output();

    let result = TestHelper::parse_stdout_to_json::<Project>(output)?;

    assert_eq!(result.id, "5f0952e3ee4c4a6f47bf696c");
    assert_eq!(result.identifier, "YIG");
    assert_eq!(result.r#type, "scrum");
    result.members.iter().find(|x| x.name == "13146781153").unwrap();
    result.members.iter().find(|x| x.name == "shaunxu1").unwrap();

    Ok(())
}