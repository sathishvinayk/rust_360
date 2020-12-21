use std::collections::HashMap;

fn reads() {
    let mut readers = HashMap::new();

    readers.insert("key1", 2);
    readers.insert("key2", 4);

    for (key, value, ) in readers {
        println!("{:?} {:?}", key, value);
    }
}

fn main() {
    let x: (i8, i8) = (1,2);
    let y: (&str, &str, &str) = ("Hi", "Howdy", "Hello");

    let _z: ( u8, bool ) = (5, true);

    // let invalid_array = [5, true];

    println!("The first value of x is {}", x.0);

    println!("{} Bob", y.2);

    // println!("invalid tuple access: {} {}", x[0], x[1]);

    reads();
}