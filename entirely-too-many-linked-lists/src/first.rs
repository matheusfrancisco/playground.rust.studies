//this is a bad stack
//#[derive(Debug)]
//pub enum List<T> {
//    Empty,
//    Elem(i32, List), // has infinite size
//}
//

// Recursive structures must be boxed, because if the definition of
// Cons looked like this:

use std::mem;

#[derive(Debug)]
pub enum ListCons<T> {
    Nil,
    Cons(T, Box<ListCons<T>>),
}

//[] = Stack
//() = Heap
//
//[Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
//We're allocating a node that just says "I'm not actually a Node"
//One of our nodes isn't heap-allocated at all.
//
//The big takeaway here is that even though Empty is a single bit of information,
//it necessarily consumes enough space for a pointer and an element, because it has to be ready to become an Elem at any time. Therefore the first layout heap allocates an extra element that's just full of junk, consuming a bit more space than the second layout.
//One of our nodes not being allocated at all is also, perhaps surprisingly,
//worse than always allocating it. This is because it gives us a non-uniform node layout. This doesn't have much of an appreciable effect on pushing and popping nodes, but it does have an effect on splitting and merging lists.
//pub enum List {
//    Empty,
//    Elem(i32, Box<List>),
//}
//

//pub enum List {
//    Empty,
//    ElemThenEmpty(i32),
//    ElemThenNotEmpty(i32, Box<List>),
//}
//

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // Moves `src` into the referenced `dest`, returning the previous `dest` value.
            // Neither value is dropped.
            // If you want to replace the values of two variables, see [`swap`](https://doc.rust-lang.org/stable/core/mem/fn.swap.html).
            // If you want to replace with a default value, see [`take`](https://doc.rust-lang.org/stable/core/mem/fn.take.html).
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        //[]

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        println!("{:?}", list);
        list.push(1);
        // head->[1]
        list.push(2);
        list.push(3);

        println!("{:?}", list);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
