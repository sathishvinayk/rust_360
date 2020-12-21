use std::io::{Error, ErrorKind};

type BoxedError = Box<dyn std::error::Error>;
type AbstractType<T> = Result<T, BoxedError>;

fn generate_errors() -> AbstractType<u8> {
    let parse_error = Error::new(ErrorKind::Other, "Generated parse errors!");

    let vec_error = Error::new(ErrorKind::InvalidData, "Vector generic error!");

    let mut vector = vec!["č", "ä"];
    vector.pop();
    // vector.pop();

    if let Some(value) = vector.last() {
        match value.parse::<u8>() {
            Ok(data) => Ok(data),
            Err(_) => return Err(BoxedError::from(parse_error))
        }
    } else {
        return Err(BoxedError::from(vec_error))
    }
}

fn main() {
    generate_errors().expect("Expecting a error");
    // println!("Multiple Error: {}", generate_errors().unwrap_err());
}