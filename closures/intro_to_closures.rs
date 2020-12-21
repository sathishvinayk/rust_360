fn main() {
    let value = |a: &str| println!("Hi am being called by {}", a);

    value("ben");
}