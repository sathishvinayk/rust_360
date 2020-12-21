fn main() {
    let value: Box<u8> = Box::new(12); //Stores on heap

    println!("Value is {:?}", value);
}