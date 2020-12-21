#[allow(unconditional_panic)]
fn custom_panic() {
    let pa = panic!("This will panic, manually invoked!");

    println!("Pa {:?}", pa);
}

#[allow(unconditional_panic)]
fn panic_2() {
    let value = 2 / 0;

    println!("{}", value);
}

fn main() {
    let data = [1,2,3,4,5];

    println!("{}", data[5]);
    custom_panic();
    panic_2();
}