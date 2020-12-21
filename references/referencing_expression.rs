fn pass_value_here(x: i8) -> i8 {
    let value: (i8, u8, u8) = (12, 43, 64);

    value.0 + x
}

fn main() {
    let anonymous = &pass_value_here(2);

    let right_hand_value = 14;

    assert_eq!(anonymous, &right_hand_value);

    let greater_than = anonymous > &10;

    println!("Greater than ?: {}", greater_than);
    println!("Anonymous value is {}", anonymous);
}