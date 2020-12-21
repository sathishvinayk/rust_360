fn test(x: i32) -> u32 {
    println!("Function call appeared: value is {}", x);

    25686
}
fn main() {
    let value = test(2);

    println!("Value returned from test function is {}", value);
}