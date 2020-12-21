fn referencing_example() {
    let mut age = 12;

    let dereference_example = &mut age;

    println!("Age reference {}", *dereference_example);
}

fn box_example() {
    let value = Box::new("age".to_string());

    println!(
        "{}", value
    );
}

fn raw_pointers() {
    let name = "Tom".to_string();

    let raw_point_name: *const String = &name;

    println!("{:?}", raw_point_name);
}

fn main() {
    referencing_example();
    box_example();
    raw_pointers();
}