fn main() {
    let mut vector: Vec<&str> = vec!["data", "value", "Some junk", "keyword"];
    let mut vector2: Vec<Vec<&str>> = vec![vec!["string"], vec!["value"]];

    let mut vector3: Vec<u8> = vec![12; 5];

    let mut vector4: Vec<char> = Vec::new();

    let mut vector5: Vec<i8> = (0..10).collect();

    let mut vector6: Vec<bool> = Vec::with_capacity(2);

    let mut vector7: Vec<i8> = vec![11, 12, 13, 14];

    vector6.push(true);
    vector6.push(false);
    vector6.push(true);
    vector6.push(false);

    vector4.push('h');
    vector4.push('e');
    vector4.push('y');

    vector.push("disk");
    vector.push("wire");
    vector.push("rotor");

    vector.pop();
    vector.pop();
    vector.pop();

    vector3.insert(2, 98);
    vector6.remove(1);

    println!("{:?}", vector);
    println!("{:?}", vector2);
    println!("{:?}", vector3);
    println!("{:?}", vector6);

    println!("The 3rd element of vector is {:?}", vector[3]);
    println!("Length of vector3 is {:?}", vector3.len());
    println!("Appending vector to vector5 is {:?}", vector5.append(&mut vector7));
    println!("Draining elements {:?}", vector5.drain(..));
    println!("Clearing all elements {:?}", vector5.clear());
    println!("IsEmpty? => {:?}", vector5.is_empty());
    println!("Reversed order of vector2 {:?}", vector2.reverse());
}