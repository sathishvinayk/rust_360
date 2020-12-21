fn match_expression() {
    let boolean = true;

    let value = match boolean {
        true => 1,
        false => 0,
    };

    println!("The value is {}", value);
}

fn main() {
    let value = vec![1,2,3,4,5];

    let status = if value.get(0).unwrap() == &1 {
        "Yes the value is one!"
    } else {
        "No its not!"
    };

    println!("Status: {}", status);
    match_expression();
}