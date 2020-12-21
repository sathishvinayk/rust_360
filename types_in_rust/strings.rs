fn concat() {
    let whole_string = vec!["This ", "is ", "a ", "string ", "literal"];

    let concaty = &whole_string.concat();

    println!("The concatenated string is, {} ", concaty);
}

fn example_2() {
    let mut name = String::new();
    name.push_str("Adam");

    println!("Name is {}", name);
}

fn string_literal() {
    let app = "Component\n is a Api running on \nGPRC";

    println!("String literal: {} is an DB", app);
    println!("This is a 
        print statement 
            with multiple lines 
                including a new line \n");

    println!("{}", r"this is a\n raw \n string");
}

fn byte_string_example() {
    let data = b"Data";

    println!("Byte generated value {:?}", data);
}

fn main() {
    let mut value = "Hello there".to_string();
    value.push_str(" am here");

    let sliced_data = &value[1..];

    let formatting_string = format!("{}", "Hi this is a formatted string");
    let sliced_formatted_value = &formatting_string[7..];

    println!("Length of string: {}", value.len());
    println!("Sliced value is: {}", sliced_data);

    println!("{}", formatting_string);
    println!("Sliced value is: {}", sliced_formatted_value);

    concat();
    example_2();
    string_literal();
    byte_string_example();
}