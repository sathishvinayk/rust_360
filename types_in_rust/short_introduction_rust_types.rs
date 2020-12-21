fn build_vector() -> Vec<u16> {
    let mut vector_empty: Vec<u16> = Vec::new();
    vector_empty.push(2i16);
    vector_empty.push(12i16);

    vector_empty
}

fn suppressed_vector() -> Vec<u16> {
    let mut vector_empty = Vec::new();
    vector_empty.push(1122);
    vector_empty.push(72);

    vector_empty
}

fn main() {
    let vector1 = build_vector();
    let vector2 = build_vector();

    println!("{:?}, {:?}", vector1, vector2);
}