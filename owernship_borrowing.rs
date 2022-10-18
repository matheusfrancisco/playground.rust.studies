

fn main() {
    let mut original = String::from("original value");
    println!("\n outer original value: \t\"{}\"", original);


    {
        
        let next = &mut original;
        *next = String::from("next value");
        
        println!("\n inner scope next: \t\"{}\"", next);
        println!("\n inner scope original: \t\"{}\"", original);
    }

    println!("\n outer original value: \t\"{}\"", original);
    
}


main()
