fn get_vector(x: &Vec<u8>) {
    println!("Vector value {:?}", x);
}

fn mutable_reference(x: &mut Vec<u8>) {
    x.push(6);
}

fn rpc() {
    let mut property = Box::new(String::from("Backyard"));

    let rp1 = &mut property;
    let rp2 = &mut property;  // Error: Cannot borrow as mutable
    let rp3 = &property;      // Error: Cannot borrow as immutable

    println!("Property values from rpc are {}, {}, {}", rp1, rp2, rp3);
}

fn main() {
    let mut value: Vec<u8> = vec![1,2,3,4,5];

    get_vector(&value);
    mutable_reference(&mut value);
    rpc();

    println!("Vector accessed here after move: {:?}", &value);
}