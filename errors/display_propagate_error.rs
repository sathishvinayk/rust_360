use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;

fn result() -> Result<u8> {
    Err(Error::from(ErrorKind::InvalidData))
}

fn tcp_stream() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(value) => println!("Value: {:?}", value),
        Err(e) => println!("Error appeared : {:?}", e)
    }
}
fn tcp_stream_2() -> Result<()>{
    let _tcp_connection = TcpStream::connect("127.0.0.1:0908")?;

    Ok(())
}

fn tcp_stream_3() {
    if let Ok(_stream) = TcpStream::connect("127.0.0.1:909512") {
        println!("Connected!")
    } else {
        println!("Not connected!")
    }
}

fn main() {
    let value = result().unwrap_err();

    println!("Normal printing mode: {}", value);
    println!("Debug error with more information: {:?}", value);

    tcp_stream();

    let value_2 = tcp_stream_2();
    println!("{:?}", value_2);

    tcp_stream_3();
}