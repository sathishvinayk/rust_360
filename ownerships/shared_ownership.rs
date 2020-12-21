use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn thread_reference_example() {
    let data: Arc<String> = Arc::new(String::from(
        "Hi this is atomically referenced!"
    ));

    data.push_str(&"So am trying to write something!"); // Error: cannot borrow data in an `Arc` as mutable

    let arched_data = Arc::clone(&data);

    thread::spawn(move || {
        println!("{:?}", arched_data);
    });

    println!("Count of atomic referenced data: {}", Arc::strong_count(&data));
}

fn main() {
    let value: Rc<Vec<u8>> = Rc::new(vec![1,2,3,4,5]);

    let value2: Rc<Vec<u8>> = value.clone();

    let value3: Rc<Vec<u8>> = Rc::clone(&value);

    println!("Original value: {:?}", value);
    println!("Reference copy value 1: {:?}", value2);
    println!("Reference copy value 2: {:?}", value3);

    println!("Count of reference: {}", Rc::strong_count(&value));

    thread_reference_example();
}