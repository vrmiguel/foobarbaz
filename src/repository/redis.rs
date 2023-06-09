use super::TaskRepository;

#[async_trait::async_trait]
impl TaskRepository for redis::Client {
    async fn delete_by_id(&self, task_id: i32) -> crate::Result {
        todo!()
    }

    async fn find_by_id(&self, task_id: i32) -> crate::Result<Option<entity::Task>> {
        todo!()
    }

    async fn insert(
        &self,
        kind: entity::TaskKind,
        run_at: sea_orm::prelude::ChronoDateTime,
    ) -> crate::Result<i32> {
        todo!()
    }

    async fn take_next_task(&self) -> crate::Result<Option<entity::Task>> {
        todo!()
    }

    async fn filter(
        &self,
        kind: entity::TaskKind,
        to_be_done: bool,
    ) -> crate::Result<Vec<entity::Task>> {
        todo!()
    }
}
