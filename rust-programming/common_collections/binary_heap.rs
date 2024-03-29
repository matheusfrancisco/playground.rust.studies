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

    // peek not remove the value
    println!("BinaryHeap Peek: {:?}", bheap.peek());
    println!("BinaryHeap: {:?}", bheap);
}

// BinaryHeap: [20, 5, 18, 1]
// BinaryHeap Max: Some(20)
// BinaryHeap: [18, 5, 1]
// BinaryHeap Peek: Some(18)
// BinaryHeap: [18, 5, 1]
