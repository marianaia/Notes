mod create_task;
mod delete_task;
mod get_tasks;
mod update_tasks;

use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};

use create_task::create_task;
use delete_task::delete_task;
use get_tasks::{get_all_task, get_one_task};
use sea_orm::DatabaseConnection;
use update_tasks::atomic_update;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/task", post(create_task))
        .route("/task/:task_id", get(get_one_task))
        .route("/task", get(get_all_task))
        .route("/task/:task_id", put(atomic_update))
        .route("/task/:task_id", delete(delete_task))
        .layer(Extension(database))
}
