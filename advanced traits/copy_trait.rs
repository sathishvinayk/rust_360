#[derive(Debug)]
struct X {
    a: i32
}

impl Copy for X { }

impl Clone for X {
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {
    let x = X {
        a: 3
    };

    let y = x;

    println!("x and y value: {:?} {:?}", x, y);
}