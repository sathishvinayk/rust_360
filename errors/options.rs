use std::error::Error;
use std::fmt;
fn find(x: f64) -> Option<u64> {
    if x == 0.0 {
        None
    } else {
        Some(x as u64)
    }
}

#[derive(Debug, PartialEq)]
struct CommonError;

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)  -> fmt::Result {
        write!(f, "Common error comes!")
    }
}
impl Error for CommonError {}

fn some_value() -> Result<Option<u32>, CommonError> {
    Ok(Some(2))
}

fn none_value() -> Result<Option<u32>, CommonError> {
    Ok(None)
}

fn return_value() -> Result<Option<u32>, CommonError> {
    Err(CommonError)
}

fn return_some() -> Option<Result<i32, CommonError>> {
    Some(Ok(2))
}

fn return_some_none() -> Option<Result<i32, CommonError>> {
    None
}

fn return_custom_err() -> Option<Result<i32, CommonError>> {
    Some(Err(CommonError))
}

fn main() {
    // println!("Some of value: {:?}", find(3.0));

    assert_eq!(find(3.0), Some(3));
    assert_eq!(find(0.0), None);

    assert_eq!(some_value().unwrap(), Some(2));

    assert_eq!(none_value().unwrap(), None);

    assert_eq!(return_value().err(), Some(CommonError));

    assert_eq!(return_some().transpose(), Ok(Some(2)));
    assert_eq!(return_some_none().transpose().err(), None);

    assert_eq!(return_custom_err().transpose().err(), Some(CommonError));
}