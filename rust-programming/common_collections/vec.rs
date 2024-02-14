use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    println!("{:?}", nums);
    let pop = nums.pop(); //returns an option Option<T>, return None or Some(T)

    println!("{:?}", pop);

    let two = nums[1]; //copy
                       // &nums[1], creates a referenc if copy is not available
                       // nums.get(1), returns an option

    println!("{}", two);
    let one = nums.first();

    println!("{:?}", one);
    //.last
    //.first_mut and .last_mut, so will borrow a mutable references
    //
    println!("{:?}", nums.len()); //val of the length
    println!("{:?}", nums.is_empty()); // bool

    nums.insert(0, 10);
    nums.insert(3, 13);
    nums.insert(2, 23);

    println!("{:?}", nums);

    nums.remove(3);
    println!("{:?}", nums);

    nums.sort();
    println!("{:?}", nums);

    nums.reverse();
    println!("{:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("{:?}", nums);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
//To change the value that the mutable reference refers to, we have to use the 
//* dereference operator to get to the value in i before we can use the += operator. 
//We’ll talk more about the dereference operator in the “Following the Pointer to the
//Value with the Dereference Operator” section of Chapter 15.

    println!("{:?}", v);
}
