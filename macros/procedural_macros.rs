use pro_mac::JunkTag;
pro_mac::function_proc!();

#[pro_mac::attribute_proc("Hi am sending a token to proc_macro")]
pub fn create_color() {
    println!("print this as a whole!");
}

#[derive(JunkTag)]
struct B;
impl B {
    fn new() {
        println!("Value from derive {}", xyz());
    }
}

fn main() {
    let a = A {
        x: 3
    };

    B::new();

    create_color();

    println!("x value is {}", a.x);
}