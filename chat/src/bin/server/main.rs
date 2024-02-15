use async_std::prelude::*;
use async_std::{net, task};
use chat::utils::ChatResult;
use connections::handle;
use std::sync::Arc;

mod chats;
mod chats_map;
mod connections;

fn log_error(result: ChatResult<()>) {
    if let Err(e) = result {
        eprintln!("{}", e);
    }
}

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address::Port");
    let chat_table = Arc::new(chats_map::ChatTracker::new());

    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(&addr).await?;

        let mut new_connections = listener.incoming();

        while let Some(socket_resutl) = new_connections.next().await {
            let socket = socket_resutl?;
            let chats = chat_table.clone();

            task::spawn(async {
                log_error(handle(socket, chats).await);
            });
        }
        Ok(())
    })
}
