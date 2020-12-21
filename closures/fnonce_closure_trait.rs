use rand;

fn return_closure() -> impl FnOnce() {
    println!("Generating some random characters: =>");
    move || {
        for _ in 0..5 {
            let n = rand::random::<char>();
    
            print!("{}, ", n);
        }
    }
}

fn generate_random<Fx>(mut closure: Fx) where Fx: FnMut() {
    closure();
}
fn main() {
    let vector_value = vec![1,2,3,4,5];
    println!("Generating some random numbers: =>");
    let random = || {
        for _ in &vector_value {
            let n = rand::random::<u16>();
    
            print!("{}, ", n);
        }
    };
    generate_random(random);
    return_closure()();
}