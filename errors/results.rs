use std::io::{Error, ErrorKind, Result};

fn call() -> Result<i8> {
    Ok(2)
}

fn return_error() -> Result<i8> {
    Err(Error::from(ErrorKind::InvalidData))
} 

fn return_success() -> Result<i8> {
    Ok(12)
}

fn is_value() {
    let is_value = return_error().is_ok();
    let is_err = return_error().is_err();
    println!("Is value ? {} ", is_value);
    println!("Is Error ? {} ", is_err);
}

fn match_call() {
    match call() {
        Ok(value) =>  println!("Value is {}", value),
        Err(e) => println!("Error appeared! {}", e)
    }

    match return_error() {
        Ok(value) => println!("Value is {}", value),
        Err(e) => println!("Error appeared {}", e)
    }

}

fn ok_or_err(){
    let success_value = return_success().ok().unwrap();
    println!("Ok ? {} ", success_value);

    // let success_error = return_success().err().unwrap();
    // println!("None ? {}", success_error);
}

fn unwrap_fallback() {
    let value = return_error().unwrap_or(5);

    println!("Unwrap ignore error: {}", value);
}

fn return_error_default() -> Result<String> {
    Err(Error::from(ErrorKind::InvalidData))
} 

fn dynamic_data(_x: std::io::Error) -> String {
    String::from("Default method")
}

fn fallback_function(){
    let value = return_error_default().unwrap_or_else(dynamic_data);

    println!("Value is {}", value);
}

fn return_unwrap() {
    // let _value = return_error_default().unwrap();
    // let expect = return_error_default().expect("Hi this is a custom error message!!");

    // println!("Value from return_unwrap{}", value);
    // println!("Value from return_unwrap{}", expect);
}

fn function_custom_return() -> Result<u8> {
    Err(Error::from(ErrorKind::NotFound))
}

fn main() {
    match_call();
    is_value();
    ok_or_err();
    unwrap_fallback();
    fallback_function();
    return_unwrap();

    let value = function_custom_return();

    println!("Utility function{:?}", value.err().unwrap());
}