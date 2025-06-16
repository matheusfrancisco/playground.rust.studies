use std::{
    cell::Cell,
    rc::{Rc, Weak},
};

use typed_arena::Arena;

struct Node {
    value: usize,
    parent: Option<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

fn build_tree(root: usize, nr_of_children: usize) -> Rc<Node> {
    Rc::new_cyclic(|parent| {
        let children = (1..=nr_of_children)
            .map(|i| {
                Rc::new(Node {
                    value: root + i,
                    parent: Some(parent.clone()),
                    children: Vec::new(),
                })
            })
            .collect();

        Node {
            value: root,
            parent: None,
            children,
        }
    })
}

struct TreeNode<'a> {
    value: i32,
    parent: Cell<Option<&'a TreeNode<'a>>>,
}

fn main() {
    let root = build_tree(10, 5);
    let nr_of_children = root.children.len();
    println!("Root has: {} children", nr_of_children);

    for child in root.children.iter() {
        if let Some(parent) = child.parent.as_ref().unwrap().upgrade() {
            println!(
                "Root has one child with value {}; its parent value is {}",
                child.value, parent.value
            );
        }
    }

    let arena = Arena::new();
    let root = arena.alloc(TreeNode {
        value: 1,
        parent: Cell::new(None),
    });
    let child_1 = arena.alloc(TreeNode {
        value: 2,
        parent: Cell::new(None),
    });
    let child_2 = arena.alloc(TreeNode {
        value: 3,
        parent: Cell::new(None),
    });
    let grandchild = arena.alloc(TreeNode {
        value: 4,
        parent: Cell::new(None),
    });

    child_1.parent.set(Some(root));
    child_2.parent.set(Some(root));
    grandchild.parent.set(Some(child_1));

    let parent = grandchild.parent.get().unwrap();
    println!("Value of grandchild's parent: {}", parent.value);

    let grandparent = parent.parent.get().unwrap();
    println!("Value of grandchild's grandparent: {}", grandparent.value);
}
