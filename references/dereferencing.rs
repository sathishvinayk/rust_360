fn vector_dereferencing() {
    let mut vector: Vec<u8> = vec![1,2,3,4,5];

    let value = &mut vector;

    value.push(6);

    println!("Updated vector values {:?}", vector);
}

fn main() {
    let mut value = 10;

    let x = &mut value;

    *x = 12;

    println!("Dereferenced value and mutated {}", value);
    vector_dereferencing();
}