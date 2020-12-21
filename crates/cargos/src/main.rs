#[derive(Debug)]
struct Axis {
    x: f32,
}

fn main() {
    let axis = Axis {
        x: 3.0
    };

    println!("{:?}", axis);
}