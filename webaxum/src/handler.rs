use crate::model::Greeting;
use std::sync::atomic::AtomicU16;

pub trait GreetingHandler: Send + Sync + 'static {
    fn greet(&self, visitor: String) -> Greeting;
    fn bye(&self) -> String;
    fn reset(&self) -> u16;
}

pub struct WebHandler {
    number_of_visitors: AtomicU16,
}

impl GreetingHandler for WebHandler {
    fn greet(&self, visitor: String) -> Greeting {
        let visits = self
            .number_of_visitors
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Greeting::new("Hello", visitor, visits)
    }

    fn bye(&self) -> String {
        "Goodbye!".to_string()
    }

    fn reset(&self) -> u16 {
        self.number_of_visitors
            .store(0, std::sync::atomic::Ordering::Relaxed);
        0
    }
}

impl Default for WebHandler {
    fn default() -> Self {
        WebHandler {
            number_of_visitors: AtomicU16::new(0),
        }
    }
}
