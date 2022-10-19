


fn main() {

    let scope_test = "outer scope";
    let scope_test = 1;
    println!("{}", scope_test); 
    {
        println!("{}", scope_test); 

    }
    println!("{}", scope_test); 
}
