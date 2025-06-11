type SampleResult<T> = std::result::Result<T, SampleError>;

#[derive(Debug, Clone)]
struct SampleError {
    pub message: String,
}

impl SampleError {
    fn new(msg: &str) -> Self {
        SampleError {
            message: msg.to_string(),
        }
    }
}

impl std::fmt::Display for SampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sample error: {}", self.message)
    }
}

impl std::error::Error for SampleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::convert::From<io::Error> for SampleError {
    fn from(io_err: io::Error) -> Self {
        SampleError {
            message: io_err.to_string(),
        }
    }
}

fn read_from_file() -> SampleResult<String> {
    match read_to_string("non_existing_file.txt") {
        Err(err) => Err(err.into()),
        Ok(content) => Ok(content),
    }
}

fn failable_function() -> SampleResult<u32> {
    Err(SampleError::new("oops"))
}

fn main() {
    if let Err(err) = failable_function() {
        println!("Error: {err}");
    }
    if let Err(err) = read_from_file() {
        println!("Error: {err}");
    }
}
