use async_trait::async_trait;
use entity::{task, Task, TaskEntity, TaskKind};
use sea_orm::{
    prelude::DateTime, sea_query::Expr, ActiveValue, DatabaseConnection,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};

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

#[async_trait]
impl TaskRepository for DatabaseConnection {
    async fn take_next_task(&self) -> crate::Result<Option<Task>> {
        // Note to Tom: The code below is susceptible to a data race but I
        // couldn't solve it with SeaORM, which seems to have serious
        // limitations on Pg transactions, not allowing me to do a `FOR UPDATE
        // SKIP LOCKED`-based queue and also seems to lack DELETE RETURNING, which
        // was my fallback plan.

        let Some(task) = TaskEntity::find()
            .order_by_asc(task::Column::TargetExecutionDateTime)
            .filter(Expr::col(task::Column::ActualExecutionDateTime).is_null())
            .filter(
                Expr::col(task::Column::TargetExecutionDateTime).lte(Expr::current_timestamp()),
            )
            .limit(1)
            .one(self)
            .await? else {
                return Ok(None);
            };

        TaskEntity::update_many()
            .col_expr(
                task::Column::ActualExecutionDateTime,
                Expr::current_timestamp().into(),
            )
            .filter(Expr::col(task::Column::Id).eq(Expr::value(task.id)))
            .exec(self)
            .await?;

        Ok(Some(task))
    }

    async fn delete_by_id(&self, task_id: i32) -> crate::Result {
        TaskEntity::delete_by_id(task_id).exec(self).await?;

        Ok(())
    }

    async fn filter(
        &self,
        kind: TaskKind,
        to_be_done: bool,
    ) -> crate::Result<Vec<Task>> {
        let select = TaskEntity::find()
            .filter(Expr::col(task::Column::Kind).eq(Expr::value(kind)));

        let select = if to_be_done {
            // Only tasks that are yet to be claimed
            select.filter(Expr::col(task::Column::ActualExecutionDateTime).is_null())
        } else {
            // Tasks already claimed
            select.filter(
                Expr::col(task::Column::ActualExecutionDateTime).is_not_null(),
            )
        };

        select.all(self).await.map_err(Into::into)
    }

    async fn find_by_id(&self, task_id: i32) -> crate::Result<Option<Task>> {
        TaskEntity::find_by_id(task_id)
            .one(self)
            .await
            .map_err(Into::into)
    }

    async fn insert(&self, kind: TaskKind, run_at: DateTime) -> crate::Result<i32> {
        let task_id = TaskEntity::insert(task::ActiveModel {
            kind: ActiveValue::Set(kind),
            target_execution_date_time: ActiveValue::Set(run_at),
            ..Default::default()
        })
        .exec(self)
        .await?
        .last_insert_id;

        Ok(task_id)
    }
}
