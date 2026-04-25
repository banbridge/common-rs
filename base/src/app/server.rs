#[async_trait::async_trait]
pub trait IServer {
    async fn start(&self);
    async fn stop(&mut self);
}
