fn reference_assignment() {
    let first = String::from("first value");
    let second: &String = &first;

    let third: &&String = &&second;

    let fourth: &&&String = &&&third;

    println!("All referenced values: {}, {}, {}, {}", first, second, third, fourth);
}

// While loop control flow
fn control_flow_while_loop() {
    let x = String::from("x value");
    let _y: i8 = 12;

    let mut original_reference = &x;
    let data = "append".to_owned();
    let mut vector = vec![1,2,];

    while vector.pop().unwrap() == 2 {
        original_reference = &data;
    }
    println!("Referenced from while loop: {}", original_reference);
}

// for loop control flow
fn control_flow_for_loop() {
    let x = String::from("x value");
    let _y: i8 = 12;

    let mut original_reference = &x;
    
    let vector = vec![1,2,3,4,5,6,7];

    for _x in vector {
        let vector = String::from("Hi am a new value");
        original_reference = &value;                // Error: cant assign since value moved.
    }

    println!("Referenced from while loop: {}", original_reference); // Value referenced after move.
}

fn main() {
    let first_variable = String::from("First value");
    let second_variable = String::from("Second value");
    let _third_variable: u8 = 12;

    let mut original = &first_variable;

    if true {
        original = &second_variable
    } else {
        println!("{}", "Nothing to assign");
    }
    println!("{}", original);

    reference_assignment();
}