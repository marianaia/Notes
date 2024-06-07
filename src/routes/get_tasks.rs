use crate::database::notes::{self, Entity as Notes};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    title: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    task: String,
    title: String,
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let note = Notes::find_by_id(task_id).one(&database).await.unwrap();

    if let Some(note) = note {
        Ok(Json(ResponseTask {
            id: note.id,
            task: note.task,
            title: note.title,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_task(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTasksQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut note_filter = Condition::all();
    if let Some(title) = query_params.title {
        note_filter = note_filter.add(notes::Column::Title.eq(title));
    }

    let notes = Notes::find()
        .filter(note_filter)
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            task: db_task.task,
        })
        .collect();

    Ok(Json(notes))
}
