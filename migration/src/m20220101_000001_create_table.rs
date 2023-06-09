use sea_orm_migration::{
    prelude::*, sea_query::extension::postgres::TypeDropStatement,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                extension::postgres::Type::create()
                    .as_enum(TaskKind::Table)
                    .values([TaskKind::Foo, TaskKind::Bar, TaskKind::Baz])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Task::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Task::TargetExecutionDateTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Task::ActualExecutionDateTime).date_time())
                    .col(
                        ColumnDef::new(Task::Kind)
                            .enumeration(
                                TaskKind::Table,
                                [TaskKind::Foo, TaskKind::Bar, TaskKind::Baz],
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(TypeDropStatement::new().name(TaskKind::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

/// The identifiers for the
#[derive(Iden)]
enum Task {
    Table,
    /// The PK for this task
    Id,
    /// When this task was set to be executed
    TargetExecutionDateTime,
    /// When this task was actually done, if it has been done
    /// already
    ActualExecutionDateTime,
    /// Task's kind (Baz, Bar or Foo)
    Kind,
}

#[derive(Iden)]
enum TaskKind {
    Table,
    Foo,
    Bar,
    Baz,
}
