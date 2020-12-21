use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[doc = "Logger function with doc attribute"]
/// # This is a description
///```ignore
///     Required:
///         - Filename &str
///         - Buffer &str
///     Returns
///         - Ok() or Err()
/// ```
/// # Test the program
/// ```
///  let result = play::time_logger("test.txt", &"Just a doc test").unwrap();
///  assert_eq!(result, ());
/// ```
pub fn time_logger(filename: &str, string: &str) -> std::io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(string.as_bytes())?;

    Ok(())
}

pub fn call_function(){ 
    let local: DateTime<Local> = Local::now();

    let formatted = local.format("%a %b %d %Y %I:%M:%S %P\n").to_string();

    match time_logger("log.txt", &formatted) {
        Ok(_) => println!("Time has been recorded!"),
        Err(_) => println!("Error has occured!")
    }
}