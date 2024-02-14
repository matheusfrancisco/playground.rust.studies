mod list {
    pub struct Tasks {
        pub items: String,
    }
}

mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;

fn lets_add_task() {
    let task = list::Tasks {
        items: String::from("Get groceries"),
    };
    add_activity(); // relative path
                    //create::list::things_todo::add_activity(); // absolute path
    items_completed::remove_task();
}

fn main() {
    lets_add_task();
}
