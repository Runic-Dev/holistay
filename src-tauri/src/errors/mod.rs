use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
struct MyError {
    message: String
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

impl Error for MyError {}
