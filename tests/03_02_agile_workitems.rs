mod helper;

use helper::TestHelper;
use helper::TestResult;
use helper::UIDS;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug)]
struct Fixture {
    pub project_id_scrum: String,
    pub epic_id: String,
    pub feature_id: String,
    pub story_id: String,
    pub task_id: String,
    pub bug_id: String,
}

static FIXTURE: Lazy<Mutex<Fixture>> = Lazy::new(|| {
    Mutex::new(Fixture {
        project_id_scrum: String::from("5f0952e3ee4c4a6f47bf696c"),
        epic_id: String::default(),
        feature_id: String::default(),
        story_id: String::default(),
        task_id: String::default(),
        bug_id: String::default(),
    })
});

#[derive(Debug, Serialize)]
struct CreateEpicRequest {
    pub project_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct Parent {
    pub id: String,
}

#[derive(Debug, Serialize)]
struct CreateFeatureStoryTaskBugRequest {
    pub project_id: String,
    pub title: String,
    pub parent_id: String,
}

#[derive(Debug, Deserialize)]
struct Assignee {
    pub id: String,
    pub url: String,
    pub name: String,
    pub display_name: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize)]
struct WorkitemResponse {
    pub id: String,
    pub project: Project,
    pub parent: Option<Parent>,
    pub title: String,
    pub r#type: String,
    pub description: Option<String>,
    pub assignee: Option<Assignee>,
    pub start_at: Option<i32>,
    pub end_at: Option<i32>,
}

#[test]
fn t01_workitem_create_epic() -> TestResult {
    let body = TestHelper::to_body_string(CreateEpicRequest {
        project_id: FIXTURE.lock().unwrap().project_id_scrum.clone(),
        title: String::from("Test Epic"),
    })?;
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("epics")
        .arg("create")
        .arg("--content")
        .arg(body)
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.project.id, FIXTURE.lock().unwrap().project_id_scrum.clone());
    assert_eq!(result.title, String::from("Test Epic"));

    FIXTURE.lock().unwrap().epic_id = result.id.clone();
    Ok(())
}

#[test]
fn t02_workitem_create_feature() -> TestResult {
    let project_id = FIXTURE.lock().unwrap().project_id_scrum.clone();
    let parent_id = FIXTURE.lock().unwrap().epic_id.clone();
    let body = TestHelper::to_body_string(CreateFeatureStoryTaskBugRequest {
        project_id: project_id.clone(),
        parent_id: parent_id.clone(),
        title: String::from("Test Feature"),
    })?;
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("features")
        .arg("create")
        .arg("--content")
        .arg(body)
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.parent.unwrap().id, parent_id.clone());
    assert_eq!(result.title, String::from("Test Feature"));

    FIXTURE.lock().unwrap().feature_id = result.id.clone();
    Ok(())
}

#[test]
fn t03_workitem_create_story() -> TestResult {
    let project_id = FIXTURE.lock().unwrap().project_id_scrum.clone();
    let parent_id = FIXTURE.lock().unwrap().feature_id.clone();
    let body = TestHelper::to_body_string(CreateFeatureStoryTaskBugRequest {
        project_id: project_id.clone(),
        parent_id: parent_id.clone(),
        title: String::from("Test User Story"),
    })?;
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("stories")
        .arg("create")
        .arg("--content")
        .arg(body)
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.parent.unwrap().id, parent_id.clone());
    assert_eq!(result.title, String::from("Test User Story"));

    FIXTURE.lock().unwrap().story_id = result.id.clone();
    Ok(())
}

#[test]
fn t04_workitem_create_task() -> TestResult {
    let project_id = FIXTURE.lock().unwrap().project_id_scrum.clone();
    let parent_id = FIXTURE.lock().unwrap().story_id.clone();
    let body = TestHelper::to_body_string(CreateFeatureStoryTaskBugRequest {
        project_id: project_id.clone(),
        parent_id: parent_id.clone(),
        title: String::from("Test Task"),
    })?;
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("tasks")
        .arg("create")
        .arg("--content")
        .arg(body)
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.parent.unwrap().id, parent_id.clone());
    assert_eq!(result.title, String::from("Test Task"));

    FIXTURE.lock().unwrap().task_id = result.id.clone();
    Ok(())
}

#[test]
fn t05_workitem_create_bug() -> TestResult {
    let project_id = FIXTURE.lock().unwrap().project_id_scrum.clone();
    let parent_id = FIXTURE.lock().unwrap().story_id.clone();
    let body = TestHelper::to_body_string(CreateFeatureStoryTaskBugRequest {
        project_id: project_id.clone(),
        parent_id: parent_id.clone(),
        title: String::from("Test Bug"),
    })?;
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("bugs")
        .arg("create")
        .arg("--content")
        .arg(body)
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.parent.unwrap().id, parent_id.clone());
    assert_eq!(result.title, String::from("Test Bug"));

    FIXTURE.lock().unwrap().bug_id = result.id.clone();
    Ok(())
}

#[test]
fn t06_workitem_get() -> TestResult {
    let project_id = FIXTURE.lock().unwrap().project_id_scrum.clone();

    let epic_id = FIXTURE.lock().unwrap().epic_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("get")
        .arg("--id")
        .arg(epic_id.clone())
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.id, epic_id.clone());
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.r#type, "epic");

    let feature_id = FIXTURE.lock().unwrap().feature_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("get")
        .arg("--id")
        .arg(feature_id.clone())
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.id, feature_id.clone());
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.r#type, "feature");

    let story_id = FIXTURE.lock().unwrap().story_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("get")
        .arg("--id")
        .arg(story_id.clone())
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.id, story_id.clone());
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.r#type, "story");

    let task_id = FIXTURE.lock().unwrap().task_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("get")
        .arg("--id")
        .arg(task_id.clone())
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.id, task_id.clone());
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.r#type, "task");

    let bug_id = FIXTURE.lock().unwrap().bug_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("get")
        .arg("--id")
        .arg(bug_id.clone())
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;
    assert_eq!(result.id, bug_id.clone());
    assert_eq!(result.project.id, project_id.clone());
    assert_eq!(result.r#type, "bug");

    Ok(())
}

#[test]
fn t07_workitem_list_workitem() -> TestResult {
    let project_id = FIXTURE.lock().unwrap().project_id_scrum.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("list")
        .arg("--project-id")
        .arg(project_id.clone())
        .output();
    let result = TestHelper::parse_list_stdout_to_json::<WorkitemResponse>(output)?;

    result.values.iter().find(|x| x.id == FIXTURE.lock().unwrap().epic_id.clone()).unwrap();
    result.values.iter().find(|x| x.id == FIXTURE.lock().unwrap().feature_id.clone()).unwrap();
    result.values.iter().find(|x| x.id == FIXTURE.lock().unwrap().story_id.clone()).unwrap();
    result.values.iter().find(|x| x.id == FIXTURE.lock().unwrap().task_id.clone()).unwrap();
    result.values.iter().find(|x| x.id == FIXTURE.lock().unwrap().bug_id.clone()).unwrap();

    Ok(())
}

#[test]
fn t08_workitem_update_epic() -> TestResult {
    let epic_id = FIXTURE.lock().unwrap().epic_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("epics")
        .arg("update")
        .arg("--id")
        .arg(epic_id.clone())
        .arg("--content")
        .arg(
            r#"
        {
            "title": "Test Epic (Updated)"
        }
        "#,
        )
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;

    assert_eq!(result.title, "Test Epic (Updated)");
    Ok(())
}

#[test]
fn t09_workitem_update_feature() -> TestResult {
    let feature_id = FIXTURE.lock().unwrap().feature_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("features")
        .arg("update")
        .arg("--id")
        .arg(feature_id.clone())
        .arg("--content")
        .arg(
            r#"
        {
            "description": "Test Feature Description"
        }
        "#,
        )
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;

    assert_eq!(result.description, Some(String::from("Test Feature Description")));
    Ok(())
}

#[test]
fn t10_workitem_update_story() -> TestResult {
    let story_id = FIXTURE.lock().unwrap().story_id.clone();
    let assignee_id = String::from(UIDS.shaunxu4);
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("stories")
        .arg("update")
        .arg("--id")
        .arg(story_id.clone())
        .arg("--content")
        .arg(format!(
            r#"
        {{
            "assignee_id": "{}"
        }}
        "#,
            assignee_id
        ))
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;

    assert_eq!(result.assignee.unwrap().id, assignee_id);
    Ok(())
}

#[test]
fn t11_workitem_update_task() -> TestResult {
    let task_id = FIXTURE.lock().unwrap().task_id.clone();
    let assignee_id = String::from(UIDS.shaunxu3);
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("tasks")
        .arg("update")
        .arg("--id")
        .arg(task_id.clone())
        .arg("--content")
        .arg(format!(
            r#"
        {{
            "assignee_id": "{}"
        }}
        "#,
            assignee_id
        ))
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;

    assert_eq!(result.assignee.unwrap().id, assignee_id);
    Ok(())
}

#[test]
fn t12_workitem_update_bug() -> TestResult {
    let bug_id = FIXTURE.lock().unwrap().bug_id.clone();
    let assignee_id = String::from(UIDS.shaunxu2);
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("bugs")
        .arg("update")
        .arg("--id")
        .arg(bug_id.clone())
        .arg("--content")
        .arg(format!(
            r#"
        {{
            "assignee_id": "{}"
        }}
        "#,
            assignee_id
        ))
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;

    assert_eq!(result.assignee.unwrap().id, assignee_id);
    Ok(())
}

#[test]
fn t13_workitem_delete() -> TestResult {
    let epic_id = FIXTURE.lock().unwrap().epic_id.clone();
    let output = helper::TestHelper::get_exe_command()
        .arg("agile")
        .arg("workitems")
        .arg("delete")
        .arg("--id")
        .arg(epic_id.clone())
        .output();
    let result = TestHelper::parse_stdout_to_json::<WorkitemResponse>(output)?;

    assert_eq!(result.id, epic_id);
    Ok(())
}