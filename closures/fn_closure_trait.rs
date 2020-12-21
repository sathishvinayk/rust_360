fn generate_name<Fx>(closure: Fx) where Fx: Fn() {
    closure();
} 

fn example() {
    let value = || println!("Hi am a closure");
    
    generate_name(value);
}

fn example1(){
    let value = || {
        let numbers: Vec<u8> = vec![124, 5, 26, 54];
        match String::from_utf8(numbers) {
            Ok(data) => println!("Data is {}", data),
            Err(_) => println!("No data!")
        }
    };

    generate_name(value);
}

fn example2() {
    let numbers: Vec<u8> = vec![124, 5, 26, 54];
    match String::from_utf8(numbers) {
        Ok(data) => println!("Data is {}", data),
        Err(_) => println!("No data!")
    }
}

fn example3() {
    let _id = 4;
    let name = String::from("David");
    let value = || println!("Name is {}", name);

    generate_name(value);
}

fn return_closure() -> impl Fn() {
    let s: Vec<_> = "A String;got seperated;because of colon".split(";").collect();

    move || println!("{:?}", s)
}

fn main() {
    example();
    example1();
    example3();
    generate_name(example2);
    return_closure()();
}
