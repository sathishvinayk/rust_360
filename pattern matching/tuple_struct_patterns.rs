struct Xor {
    x: i32,
    y: i32,
}
enum XorBinary {
    Binary(i32, i32)
}
fn get_pattern(x: i8, y: i8) -> i8 {
    // let a, b = (x, y); //Error unexpected pattern
    // let (a, b) = x, y; //Error unexpected pattern
    // let (a, b, c) = x, y; //Error unexpected pattern
    // let (a, b, c) = (x, y); //Error unexpected pattern
    // Pattern 1
    // let data = (x, y); 
    // let (a, b) = data;
    // match (a, b) {
    //     (1, 0) | (0, 1) => 1,
    //     _ => 0
    // }
    // Pattern 2
    // match data {
    //     (1, 0) | (0, 1) => 1,
    //     _ => 0
    // }
    match (x, y) {
        (1, 0) | (0, 1) => 1,
        _ => 0
    }
}

fn get_via_struct(data: Xor) -> i8 {
    // Pattern 1
    // let (a, b) = (data.x, data.y);
    // match (a, b) {
    //     (1, 0) | (0, 1) => 1,
    //     _ => 0
    // }
    // Pattern 2
    // let Xor { x, y } = data;
    // match (x, y) {
    //     (1, 0) | (0, 1) => 1,
    //     _ => 0
    // }
    // Pattern 3
    let Xor {x: a, y: b } = data;
    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0
    }
}

fn get_via_enum(data: XorBinary) -> i8 {
    let XorBinary::Binary(a, b) = data;

    match (a, b) {
        (1, 0) | (0, 1) => 1,
        _ => 0
    }
}

fn use_if_let(x: i32, y: i32) -> i8 {
    if let (1, 0) | (0, 1) = (x, y) {
        1
    } else {
        0
    }
}

fn main() {
    let xor = get_pattern(1, -1);

    let xor_struct = Xor {x: -1, y: -1 };

    let xor_enum = XorBinary::Binary(1, 1);

    let if_let_value = use_if_let(1, 0);

    println!("Xor: {}", xor);
    println!("Xor struct value: {:?}", get_via_struct(xor_struct));

    println!("Xor enum value: {:?}", get_via_enum(xor_enum));
    println!("If let value: {}", if_let_value);
}
