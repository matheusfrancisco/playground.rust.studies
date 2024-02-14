use std::sync::mpsc;

fn main() {
    let (transmitter, receiver) = mpsc::channel();
    //let (transmitter, receiver) = mpsc::sync_channel(1000);
    let tx = transmitter.clone();

    //let val = String::from("Transmitter");

    //std::thread::spawn(move || {
    //  transmitter.send(val).unwrap();
    //});

    //let msg = receiver.recv().unwrap();
    //println!("Message: {}", msg);
    //
    std::thread::spawn(move || {
        let vec = vec![
            String::from("Transmitter"),
            String::from("From"),
            String::from("Original"),
        ];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });

    std::thread::spawn(move || {
        let vec = vec![
            String::from("Clone"),
            String::from("is"),
            String::from("Transmitter"),
        ];
        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in receiver {
        println!("Message: {}", rec);
    }
}
