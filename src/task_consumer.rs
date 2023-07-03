use std::time::Duration;

use entity::Task;
use sea_orm::DatabaseConnection;
use tokio::{select, sync::mpsc::UnboundedSender, task::JoinHandle};

use crate::repository::TaskRepository;

/// Check the DB at a fixed interval to see if there are new tasks to be done.
///
/// Note to Tom: If SeaORM supported `pg_notify` (as sqlx does, for example), we
/// could have an listen async for new Pg updates, but that's not an option so
/// far with this library
pub struct TaskConsumer {
    sender: UnboundedSender<Task>,
    db_conn: DatabaseConnection,
}

impl TaskConsumer {
    async fn poll_for_new_tasks(self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let task = match self.db_conn.take_next_task().await {
                Ok(None) => continue,
                Ok(Some(task)) => {
                    tracing::info!("Got a new task, sending to worker");
                    task
                }
                Err(err) => {
                    tracing::error!("TaskConsumer got an error: {err}. Continuing.");
                    continue;
                }
            };

            let Ok(()) = self.sender.send(task) else {
                // Send will only ever fail if the receiving end of the channel drops, which
                // shouldn't happen
                tracing::error!("Task receiver dropped unexpectedly. Exiting.");
                break;
            };
        }
    }

    pub fn spawn(
        sender: UnboundedSender<Task>,
        db_conn: DatabaseConnection,
    ) -> JoinHandle<()> {
        let this = Self { sender, db_conn };
        let polling_loop = async move { this.poll_for_new_tasks().await };

        tokio::spawn(async {
            select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("SIGINT received, closing TaskConsumer");
                },
                _ = polling_loop => {
                    tracing::warn!("TaskConsumer closed!");
                }
            }
        })
    }
}
