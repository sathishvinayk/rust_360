use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct CommonError;

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Common error comes from here!")
    }
}

impl Error for CommonError {}

fn get_value() -> Result<(), CommonError> {
    Err(CommonError)
}

fn main() {
    match get_value() {
        Err(e) => println!("{}", e),
        _ => println!("No error")
    }
}