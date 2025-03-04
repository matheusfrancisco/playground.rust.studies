use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.events";

    println!("Connecting to {}", url);

    let (ws_stream, _) = connect_async(url).await.unwrap();
    println!("Connected");

    let (mut write, mut read) = ws_stream.split();
    let msg = Message::Text("Hello WebSocket".into());

    write.send(msg).await.expect("Failed to send message");

    if let Some(message) = read.next().await {
        let message = message.expect("failed to read");
        println!("Received: {}", message);
    }

    let msg = Message::Text("Hello WebSocket".into());
    write.send(msg).await.expect("Failed to send message");

    if let Some(message) = read.next().await {
        let message = message.expect("failed to read");
        println!("Received: {}", message);
    }
}
