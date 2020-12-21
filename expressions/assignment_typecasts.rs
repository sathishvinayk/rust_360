fn invalid_assignment() {
    let _b = 2;
    // let _a = b = 3;

    let data = 0;

    // data++
    // data--
    // println!("{:?}, {:?}", a, b);
}

fn type_casting() {
    let value: u8 = 12;
    let new_value = value as i8;

    println!("Type casting: {:?}", new_value);

    let floating_point = 3.9;

    let new_value2 = floating_point as i8;

    println!("Floating point type casting: {:?}", new_value2);

    let char_type = 'č';

    let char_value = char_type as u8;

    println!("Charc value: {}", char_value);

    let data: bool = true;
    let new_boolean = data as u8;

    println!("Boolean value: {}", new_boolean);

    let boolean_ds: i8 = 12;

    let invalid_casting = boolean_ds as bool; //Error: Cannot cast to boolean

    println!("Invalid casting: {}", boolean_ds);
}

fn invalid_char_casting() {
    let value: i8 = 12;

    let new_value = value as char; //Error: only `u8` can be cast as `char`, not `i8`

    println!("Invalid character casting {}", new_value);
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    let mut z = 3;
    let mut a = 4;

    x += 2;
    y -= 2;
    y *= 3;
    y /= 1;

    println!("Compound assignment: {}, {}, {}, {}", x, y, z, a);

    type_casting();
    invalid_char_casting();
}