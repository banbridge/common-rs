use std::sync::Arc;

use axum::Router;
use log::info;
use tokio::{net::TcpListener, sync::mpsc};

use super::IServer;

pub struct AxumServer {
    router: Router,
    port: u32,
    shutdown_sender: mpsc::Sender<()>,
    shutdown_receiver: Arc<mpsc::Receiver<()>>,
}

impl AxumServer {
    pub fn new(router: Router, port: u32) -> Self {
        let (shutdown_sender, shutdown_receiver) = mpsc::channel::<()>(1);
        Self {
            router,
            port,
            shutdown_sender,
            shutdown_receiver: Arc::new(shutdown_receiver),
        }
    }
}

#[async_trait::async_trait]
impl IServer for AxumServer {
    async fn start(&self) {
        let listener = TcpListener::bind(format!("0.0.0.0:{:}", self.port))
            .await
            .unwrap();

        info!("admin service start at: {}", self.port);

        axum::serve(listener, self.router.clone())
            .with_graceful_shutdown(shut_down(Arc::clone(&self.shutdown_receiver)))
            .await
            .unwrap();
    }

    async fn stop(&mut self) {
        self.shutdown_sender.send(()).await.unwrap()
    }
}

pub(super) async fn shut_down(rv: Arc<mpsc::Receiver<()>>) {
    let mut rv = Arc::try_unwrap(rv).unwrap();
    rv.recv().await;
}
