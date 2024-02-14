// Errors handling
/*
*Rust groups errors into two major categories: recoverable and unrecoverable errors.
For a recoverable error, such as a file not found error, we most likely just
want to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array, and so we want to immediately stop the program.

Most languages don’t distinguish between these two kinds of errors and handle both
in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions.
Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that
stops execution when the program encounters an unrecoverable error. This chapter covers calling
panic! first and then talks about returning Result<T, E> values.
Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.
* */
//enum Result<T, E> {
//   Ok(T),
//  Err(E),
// }
// the result enum
//

use std::fs::rename;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
fn main() {
    /*this can return an result with error
    * let file = File::open("hello.txt");
    * let file = match file {
        Ok(file) => file,
        //Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };
    let file = File::open("hello.txt").unwrap();
    unwrap will call panic for us
    customize
    let file = File::open("hello.txt").expect("Error opening the file");
    */

    // propagate errors
    let test = open_file();

    //test.unwrap();
    //rename_file().unwrap();
    // alternative to use match
    //
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn open_file() -> Result<File, Error> {
    let f = File::open("hello.txt")?;
    Ok(f)
}

fn rename_file() -> Result<(), Error> {
    let file = rename("error.txt", "world.txt")?;
    Ok(file)
}
