use tokio_cron_scheduler::JobScheduler;

use super::IServer;

pub struct CronServer {
    scheduler: JobScheduler,
}

impl CronServer {
    pub fn new(scheduler: JobScheduler) -> CronServer {
        CronServer { scheduler }
    }
}

#[async_trait::async_trait]
impl IServer for CronServer {
    async fn start(&self) {
        self.scheduler.start().await.unwrap();
    }

    async fn stop(&mut self) {
        self.scheduler.shutdown().await.unwrap();
    }
}
