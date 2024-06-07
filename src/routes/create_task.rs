use crate::database::notes;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    task: String,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> impl IntoResponse {
    let new_task = notes::ActiveModel {
        task: Set(request_task.task.clone()),
        title: Set(request_task.title.clone()),
        ..Default::default()
    };

    match new_task.save(&database).await {
        Ok(_) => StatusCode::CREATED,
        Err(_err) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
