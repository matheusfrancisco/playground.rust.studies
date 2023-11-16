
//  BinaryHeap
use std::collections::BinaryHeap;

fn main() {
    let mut bheap = BinaryHeap::new();
    bheap.push(1);
    bheap.push(18);
    bheap.push(20);
    bheap.push(5);

    println!("BinaryHeap: {:?}", bheap);
    let max = bheap.pop();
    println!("BinaryHeap Max: {:?}", max);
    println!("BinaryHeap: {:?}", bheap);
    println!("BinaryHeap Peek: {:?}", bheap.peek());
    println!("BinaryHeap: {:?}", bheap);
}
