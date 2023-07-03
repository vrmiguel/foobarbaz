use actix_web::{
    get, post,
    web::{self, Json},
};
use entity::Task;
use sea_orm::DatabaseConnection;
use tracing::instrument;

use crate::{
    api::forms::{CreateTask, CreateTaskResponse, ListTasks},
    error::Error,
    repository::TaskRepository,
    Result,
};

#[instrument(skip(db_conn))]
#[get("/list")]
pub async fn list_tasks(
    web::Json(ListTasks { kind, to_be_done }): web::Json<ListTasks>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<Json<Vec<Task>>> {
    let tasks = db_conn.filter(kind, to_be_done).await?;

    Ok(Json(tasks))
}

#[instrument(skip(db_conn))]
#[post("/create")]
pub async fn create_task(
    web::Json(CreateTask { kind, run_at }): web::Json<CreateTask>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<Json<CreateTaskResponse>> {
    let task_id = db_conn.insert(kind, run_at).await?;

    Ok(Json(CreateTaskResponse { task_id }))
}

#[instrument(skip(db_conn))]
#[get("/get/{task_id}")]
pub async fn get_task(
    path: web::Path<i32>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<Json<Task>> {
    let task_id = path.into_inner();

    let task = db_conn.find_by_id(task_id).await?.ok_or(Error::NotFound)?;

    Ok(Json(task))
}

#[post("/delete/{task_id}")]
pub async fn delete_task(
    path: web::Path<i32>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<Json<()>> {
    db_conn.delete_by_id(path.into_inner()).await?;

    Ok(Json(()))
}
