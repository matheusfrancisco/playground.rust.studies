use std::fs::OpenOptions;

pub fn file_open_options() -> std::io::Result<()> {
    let f = OpenOptions::new()
        .read(true) // open for reading
        .write(true) // open for writing
        .create(true) // create the file if it doesn't exist
        .append(true) // append to the end of the file
        .open("path.txt")?; // open the file
    //
    Ok(())
}

pub fn file_systems() -> std::io::Result<()> {
    // Rust provides type-safe variants of str
    // and String in its standard library: std::path:: Path and std::path::PathBuf

    let hello_buf = std::path::PathBuf::from("/tmp/hello.txt");
    let hello = std::path::Path::new("/tmp/hellopath.txt");
    let ex = hello.extension();
    println!("Extension: {:?}", ex);

    //s an implementation detail, std::fs::Path and std::fs::PathBuf are implemented on top of std::ffi::OsStr and std::ffi::OsString, respectively.
    //This means that Path and PathBuf are not guaranteed to be UTF-8 compliant.

    // why use path instead manipulate the strings directly?
    // Clear intent—Path provides useful methods like set_extension()
    // that describe the intended outcome. This can assist programmers 
    // who later read the code. Manipulating strings doesn’t provide 
    // that level of self-documentation.
    // Easier debugging —Path provides a Debug implementation


    Ok(())
}

