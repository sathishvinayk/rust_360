use std::thread;
use std::collections::HashMap;

fn function_example() {
    let mut location = HashMap::new();
    location.insert("City", 1);
    location.insert("district", 2);
    location.insert("nation", 3);

    thread::spawn(move || {
        let value = location.get("City").ok_or("key invalid");

        println!("The value is {}", value.unwrap());
    });
}

fn function_example2() {
    let value:u8 = 2;

    thread::spawn(move || {
        println!("The value is {}", value);
    });
    println!("After closure the value is {}", value);
}

fn main() {
    let name = String::from("bob");

    let print_name = move || println!("The name is {}", name);

    print_name();
    function_example();
    function_example2();
    // println!("Accessing after moving {}", name);
}