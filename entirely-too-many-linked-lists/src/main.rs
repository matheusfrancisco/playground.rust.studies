use entirely_too_many_linked_lists::first::ListCons;

fn main() {
    let list: ListCons<i32> = ListCons::Cons(
        1,
        Box::new(ListCons::Cons(2, Box::new(ListCons::Cons(3, Box::new(ListCons::Nil))))),
    );
    println!("{:?}", list);
}
