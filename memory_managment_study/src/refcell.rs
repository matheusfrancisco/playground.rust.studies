use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}

struct AuthService {
    db: Rc<RefCell<Database>>,
}
struct ContentService {
    db: Rc<RefCell<Database>>,
}

fn main() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));

    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };
    let mut r1 = db.borrow_mut();
    r1.max_connections = 200;
}
