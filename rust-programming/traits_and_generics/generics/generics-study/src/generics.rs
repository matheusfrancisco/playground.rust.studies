struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn serialize_payload<T>(payload: T) -> String {
    "test".to_owned()
}

fn main() {
    let cmd = BrowserCommand {
        name: "open".to_string(),
        payload: "https://www.rust-lang.org".to_string(),
    };

    let cmd2 = BrowserCommand::new("open".to_string(), "https://www.rust-lang.org".to_string());
    cmd2.print_payload();
}

//main()
