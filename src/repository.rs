mod postgres;
mod redis;

use async_trait::async_trait;
use entity::{Task, TaskKind};
use sea_orm::prelude::DateTime;

/// Defines behavior for modifying/storing/listing tasks from
/// some repository.
///
/// We can then implement this trait for different backends, e.g.
/// `PostgresTaskRepository`, `RedisTaskRepository` and so on.
///
/// This is very useful for testing since we're then able to
/// implement a `MockTaskRepository` and then run tests without
/// needing an external service up and running, as well as
/// allowing for easier refactorings if we ever need to change
/// the backend for any reason.
#[async_trait]
pub trait TaskRepository {
    /// Delete a Task by its ID
    async fn delete_by_id(&self, task_id: i32) -> crate::Result;

    /// Find a Task by its ID
    async fn find_by_id(&self, task_id: i32) -> crate::Result<Option<Task>>;

    /// Insert a new Task with the given kind and start time in the database,
    /// returning its id
    async fn insert(&self, kind: TaskKind, run_at: DateTime) -> crate::Result<i32>;

    /// Take the next available task, if any
    async fn take_next_task(&self) -> crate::Result<Option<Task>>;

    /// List tasks by their kind and by whether they were already claimed
    async fn filter(
        &self,
        kind: TaskKind,
        to_be_done: bool,
    ) -> crate::Result<Vec<Task>>;
}
