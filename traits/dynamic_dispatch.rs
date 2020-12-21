use std::fmt::Display;

struct Triangle {
    a: u8,
    b: u8,
    c: u8
}

struct Square {
    side: u8
}

trait Shape: Display {
    fn perimeter(&self) -> u8;
}

impl Shape for Square {
    fn perimeter(&self) -> u8 {
        4 * (self.side)
    }
}
impl Shape for Triangle {
    fn perimeter(&self) -> u8 {
        self.a + self.b + self.c
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.perimeter())
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.perimeter())
    }
}

fn logd(shape: &dyn Shape) {
    println!("Perimeter of shape: {}", shape);
}

fn main() {
    let triangle = Triangle {
        a: 2,
        b: 2,
        c: 2
    };
    let square = Square {
        side: 4
    };

    let mut shapes: Vec<&dyn Shape> = Vec::new();
    shapes.push(&triangle);
    shapes.push(&square);

    for shape in shapes {
        logd(shape)
    }

    let boxed_shape: Box<dyn Shape> = Box::new(triangle);
    println!("Perimeter from box triat: {}", boxed_shape);
}