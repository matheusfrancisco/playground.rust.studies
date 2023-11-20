use crate::connection::Leaving;
use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast;

use chat::Server;
use tokio::sync::broadcast::error::RecvError;

pub struct Chats {
    name: Arc<String>,
    publiser: broadcast::Sender<Arc<String>>,
}

impl Chats {
    pub fn new(name: Arc<String>) -> Self {
        let (publiser, _) = broadcast::channel(1000);
        Self { name, publiser }
    }

    pub fn join(&self, leaving: Arc<Leaving>) {
        let rx = self.publiser.subscribe();
        let name = self.name.clone();
        task::spawn(sub(name, rx, leaving));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ = self.publiser.send(message);
    }
}

async fn sub(
    chat_name: Arc<String>,
    mut rx: broadcast::Receiver<Arc<String>>,
    leaving: Arc<Leaving>,
) {
    loop {
        let packet = match rx.recv().await {
            Ok(message) => Server::Message {
                chat_name: chat_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => Server::Error(format!("Dropped {} messages", n)),
            Err(RecvError::Closed) => break,
        };
        if leaving.send(packet).await.is_err() {
            break;
        }
    }
}
