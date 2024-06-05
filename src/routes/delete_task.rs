use crate::db::notes::Entity as Quote;
use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    Quote::delete_by_id(task_id)
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
