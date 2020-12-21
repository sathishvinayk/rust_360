struct TupleLike(u8, u8, i8, char, String, u32, Vec<u8>);

fn main() {
    let value = TupleLike(2, 3, -2, 'ç', "name".to_string(), 24, vec![2]);

    // println!("Accessing tuple value: {}", value.0); 

    assert_eq!(value.0, 2);
    assert_eq!(value.1, 3);
    assert_eq!(value.2, -2);
    assert_eq!(value.3, 'ç');
    assert_eq!(value.4, "name".to_owned());
    assert_eq!(value.5, 24);
    assert_eq!(value.6, [2]);
}