#Error executing vim.schedule lua callback: ...im-linux64/share/nvim/runtime/lua/vim/lsp/inlay_hint.lua:377: filter: expected table, got number
[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1
}

fn print_all(items: Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main_1() {
    let mut item = Item { count: 1 };
    println!("{:?}", item);
    add_one(&mut item);
    println!("{:?}", item);
}

fn mainz_1() {
    let mut items = vec![Item { count: 1 }];
    let first = items.first_mut();
    println!("{:?}", first);
    print_all(items);
    //println!("{:?}", first);
}

fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.get_mut(0);
    let second = items.get_mut(1);
    println!("{:?}", second);
}

// there are three rules you must have in your head at all times.
// there can only be one value owner
// there can be unlimited immutable borrows(reference) with no mutable references
// there can be only one mutable reference and no immutable references
