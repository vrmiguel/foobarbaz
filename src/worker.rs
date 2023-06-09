use entity::{Task, TaskKind};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::{Duration, Instant},
};

/// Receives tasks and performs them according to their time.
///
/// Defers slow tasks to separate [green threads](tokio::task) in order to keep
/// the main accept loop with very small latency
pub struct Worker {
    http_client: reqwest::Client,
    receiver: UnboundedReceiver<Task>,
}

impl Worker {
    /// Generate a random number, N (0 to 343 inclusive) and print "Baz {N}"
    fn handle_baz(&self) {
        println!("Baz {}", fastrand::u16(0..=343));
    }

    /// Make a GET request to https://www.whattimeisitrightnow.com/ and print the response's status code
    fn handle_bar(&self) {
        // Cheap clone since it's an Arc clone
        let client = self.http_client.clone();

        let work = async move {
            let _ = client
                .get("https://www.whattimeisitrightnow.com/")
                .send()
                .await
                .map(|response| {
                    tracing::info!("Finished Bar");
                    println!("Bar {}", response.status());
                })
                .map_err(|err| tracing::error!("Failed HTTP request: {err}"));
        };

        // Defer to another task since an HTTP request could take significant time
        tokio::spawn(work);
    }

    /// The worker should sleep for 3 seconds, and then print "Foo {task_id}".
    fn handle_foo(&self, task_id: i32) {
        let target = Instant::now() + Duration::from_secs(3);

        // We use `sleep_until` instead of `sleep` to account for the possible
        // (albeit very small) overhead of `tokio::spawn`
        let work = async move {
            tokio::time::sleep_until(target).await;
            tracing::info!("Worker finished `Foo {task_id}`");
            println!("Foo {task_id}");
        };

        // Defer to another task since we don't want to block the worker for 3
        // seconds
        tokio::spawn(work);
    }

    pub async fn perform_tasks(mut self) {
        while let Some(task) = self.receiver.recv().await {
            match task.kind {
                TaskKind::Baz => {
                    // This is the fastest task type so we just perform it
                    // directly
                    self.handle_baz();
                }
                TaskKind::Bar => {
                    self.handle_bar();
                }
                TaskKind::Foo => self.handle_foo(task.id),
            }
        }

        // We'll only get here if the Sender was dropped
    }

    /// Spawns the worker green thread, returning its JoinHandle and the Sender
    /// that will send it messages
    pub fn spawn() -> (JoinHandle<()>, UnboundedSender<Task>) {
        let (sender, receiver) = unbounded_channel();

        let worker = Worker {
            http_client: reqwest::Client::new(),
            receiver,
        };

        let handle = tokio::spawn(worker.perform_tasks());

        (handle, sender)
    }
}
