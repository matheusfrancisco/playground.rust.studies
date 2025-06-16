use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    message: String,
    visitor: String,
    visits: u16,
}

impl Greeting {
    pub(crate) fn new(message: &str, visitor: String, visits: u16) -> Self {
        Greeting {
            message: message.to_string(),
            visitor,
            visits,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OurJwtPayload {
    pub sub: String,
    pub exp: usize,
}

impl OurJwtPayload {
    pub fn new(sub: String) -> Self {
        // expires by default in 60 minutes from now
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(60 * 60))
            .expect("valid timestamp")
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("valid duration")
            .as_secs() as usize;

        OurJwtPayload { sub, exp }
    }
}
