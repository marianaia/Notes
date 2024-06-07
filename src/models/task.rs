use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    pub title: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseTask {
    pub id: i32,
    pub task: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: i32,
    pub task: String,
    pub title: String,
}
