fn main() {
    let mut values: [u8; 5] = [4, 5, 76, 1, 7];

    let values2 = ["true"; 5];

    values.sort();

    println!("{:?}", values);
    println!("{:?}", values2);
}