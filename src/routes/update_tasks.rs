use crate::database::notes::{self, Entity as Notes};
use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: i32,
    pub task: String,
    pub title: String,
}

pub async fn atomic_update(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let update_task = notes::ActiveModel {
        id: Set(task_id),
        task: Set(request_task.task),
        title: Set(request_task.title),
        ..Default::default()
    };

    Notes::update(update_task)
        .filter(notes::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
