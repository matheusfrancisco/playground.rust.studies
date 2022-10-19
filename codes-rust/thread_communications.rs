use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let (john_tx, john_rx) = mpsc::channel();
    let (sarah_tx, sarah_rx) = mpsc::channel();

    let john_h = thread::spawn(move || {
        john_chat(sarah_tx, john_rx);
    });
    let sarah_h = thread::spawn(move || {
        sarah_chat(john_tx, sarah_rx)
    });

    match john_h.join() {
        Ok(_) => {}
        Err(_) => {}
    }

    match sarah_h.join() {
        Ok(_) => {}
        Err(_) => {}
    }
}

fn sarah_chat(john_tx: Sender<&str>, sarah_rx: Receiver<&str>) {
    let r = sarah_rx.recv();
    println!("{}", r.unwrap());

    let _send_result = john_tx.send("Hello John.");
}

fn john_chat(sarah_tx: Sender<&str>, john_rx: Receiver<&str>) {
    let r = john_rx.recv();
    println!("{}", r.unwrap());

    let _send_result = sarah_tx.send("Hello Sarah.");
}
