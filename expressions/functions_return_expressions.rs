fn f1(_x:i32) {
    // x + 4
    return
}

fn f2(x: u32) -> Vec<u32> {
    let mut first_varaible = Vec::<u32>::with_capacity(x as usize);

    first_varaible.push(2);

    first_varaible
}

fn f3() -> Vec<u32> {
    let mut vector_turbo = (0..10).collect::<Vec<u32>>();

    vector_turbo.push(2);

    vector_turbo
}

fn main() {
    let value = f1(2);

    println!("Value {:?}", value);

    let vector = f2(4);

    println!("Vector Value is {:?}", vector);

    println!("Vector Value is {:?}", f3());
    
}