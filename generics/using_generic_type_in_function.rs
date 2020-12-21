use std::io::prelude::*;
use std::io::{self};
use flate2::Compression;
use flate2::write::ZlibEncoder;

use std::fmt::Display;

fn encoder(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(data)?;
    encoder.finish()
}

fn accept_anywhere<U: Display>(_value: U) {
    let data = _value;
    println!("Printing {}", data);
}
fn main() { 
    let value:u8 = 12;
    let value2: char = '3';
    let string_phrase = b"Hi this is phrase that will get encoded and decoded by flate2";

    println!("String as u8 bytes {:?}", string_phrase.as_ref());

    accept_anywhere(value);
    accept_anywhere(value2);
    accept_anywhere::<String>(String::from("name"));

    let encode_string = encoder(string_phrase.as_ref()).expect("Unable to encode the phrase");

    println!("Encoding the string : {:?}", encode_string);
}