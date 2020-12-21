fn value(x: &[u32]) {
    for data in x {
        println!("Value from slice: {}", data)
    }
}

fn main() {
    let data: Vec<u32> = vec![1,2,3,4,465];
    let array_data: [u32; 4] = [2,3,568,123];

    let array_slice: &[u32] = &array_data[2..];
    let vector_slice: &[u32] = &data[2..];

    value(array_slice);
    value(vector_slice);

    println!("Indexed slice vector value {:?}", vector_slice);
    println!("Indexed slice array value {:?}", array_slice);
}