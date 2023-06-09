use entity::TaskKind;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub kind: TaskKind,
    pub run_at: DateTime,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListTasks {
    pub kind: TaskKind,
    /// If true, only include tasks that are yet to be executed
    pub to_be_done: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskResponse {
    pub task_id: i32,
}
