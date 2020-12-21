fn _generic_f<T: Sized>(_x: T) -> u8 {
    2
}
fn generic_fn<T: ?Sized>(_x: &T) -> u8 {
    3
}

fn main() {
    let a = generic_fn("abc");
    println!("{}", a);
}