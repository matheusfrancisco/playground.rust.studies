use crate::{
    handler::{GreetingHandler, WebHandler},
    web::AxumWebServer,
};

mod handler;
mod model;
mod web;

#[tokio::main]
async fn main() {
    // Create a shared state for our application. We use an Arc so that we clone the pointer to the state and
    // not the state itself.
    // not the state itself. The AtomicU16 is a thread-safe integer that we use to keep track of the number of visits.
    let handler = WebHandler::default();
    let web_server = AxumWebServer::new(handler);
    web_server.start().await;
}
