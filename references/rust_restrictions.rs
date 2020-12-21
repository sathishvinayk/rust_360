fn restriction1() {
    let data = String::from("Creating new id");

    {
        let shared_reference = &data;
        shared_reference.to_uppercase();
        println!("Updated: {}", shared_reference);
    }
    let try_to_move = data;

    println!("Moved data: {}", try_to_move);
}

fn restriction2() {
    let value: i8 = 12;

    let mut y = &mut value;
}

fn restriction3() {
    let mut value: u8 = 52;

    let _y = &value;
    let _z = &value;

    let _a = &mut value;

    // println!("From restriction3 => {}, {}", y, z);
}

fn restriction4() {
    let mut x = String::from("Junk values here");

    let y = &x;
    let z = &x;

    // x.push_str(" adding more junk");

    println!("{}, {}", y, z);
}

fn restriction5() {
    let mut x = String::from("Junk again");

    let y = &mut x;
    let z = &mut x;

    let move_x_here = x;

    println!("{}, {}", y, z);
}

fn main() {
    restriction1();
    restriction2();
    restriction3();
    restriction4();
    restriction5();
}