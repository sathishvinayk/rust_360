fn example1() {
    let value = 3;

    match value {
        1 | 3 | 5 | 7 => println!("Odd value matches!!"),
        11..=19 => println!("Recieved value is from 11 to 19!!"),
        _ => println!("Value is either a even number or greater than 10")
    }
}
fn example2() {
    let data = 'g';

    match data {
        'a'..='z' | 'A'..='Z' => println!(" Its a alphabet"),
        _ => println!("Recieved difference characters")
    }
}

fn example3() {
    let pattern_at = 9;
    match pattern_at {
        info @ 0..=9 => println!("The matched value is {}!!", info),
        _ => println!("The value doen't fall under the creteria!")
    }
}

fn example4() {
    const START: u32 = 9;
    const END: u32 = 15;

    if let pat@START..=END = 12 {
        println!("Value, {} exist within the range!", pat)
    }
}

fn example5() {
    let value = Some(4);

    match value {
        Some(ux) if ux * 4 == 16 => println!("Condition matches!"),
        Some(ux) => println!("Condition doesn't match with value {}", ux),
        None => println!("No value provided!")
    }
}

fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
}