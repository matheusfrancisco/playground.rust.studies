use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{io, net, task};
use std::sync::Arc;

use chat::utils::{self, ChatResult};
use chat::{Client, Server};

fn get_value(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim();
    if input.is_empty() {
        return None;
    }
    match input.find(char::is_whitespace) {
        Some(index) => Some((&input[0..index], &input[index..])),
        None => Some((input, "")),
    }
}

fn parse_input(line: &str) -> Option<Client> {
    let (input, remainder) = get_value(line)?;
    if input == "join" {
        let (chat, remainder) = get_value(remainder)?;

        if !remainder.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Join {
            chat_name: Arc::new(chat.to_string()),
        });
    } else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        if !remainder.trim_start().is_empty() {
            return None;
        }
        return Some(Client::Post {
            chat_name: Arc::new(chat.to_string()),
            message: Arc::new(remainder.to_string()),
        });
    } else {
        println!("Invalid input: {}", input);
        None
    }
}

async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Options: \njoin Chat\n post Chat Message");

    let mut options = io::BufReader::new(io::stdin()).lines();

    while let Some(options_result) = options.next().await {
        let opt = options_result?;
        let req = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };

        utils::send_json(&mut send, &req).await?;
        send.flush().await?;
    }
    Ok(())
}

async fn messages(server: net::TcpStream) -> ChatResult<()> {
    let buf = io::BufReader::new(server);

    let mut stream = utils::receive(buf);

    while let Some(msg) = stream.next().await {
        match msg? {
            Server::Message { chat_name, message } => {
                println!("Chat Name: {}\n, Message: {}\n", chat_name, message);
            }
            Server::Error(message) => {
                println!("Message: {}\n", message);
            }
        }
    }
    Ok(())
}

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address::Port");
    task::block_on(async {
        let sck = net::TcpStream::connect(addr).await?;
        sck.set_nodelay(true)?;

        let send = send(sck.clone());
        let replies = messages(sck);

        replies.race(send).await?;
        Ok(())
    })
}
