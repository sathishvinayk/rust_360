#[derive(Debug)]
struct Tablet {
    name: String,
    boiling_point: f32
}

fn pointer_reference() -> (u8, u8) {
    let x = &2;

    let y = &10;

    let referenced_x = match x {
        &val => val
    };

    let referenced_y = match *y {
        val => val
    };

    (referenced_x, referenced_y)
}

fn pointer_arguments(x: &i32, y: &i32) {
    let &(x1, y1) = &(x, y);

    match (x1, y1) {
        (&x1, &y1) => println!("Value of x1: {} and y1: {}", x1, y1)
    }
}

fn string_value(x: &str) -> String {
    match x {
        "expressionless" => "\u{1F611}".to_owned(),
        "monocle" => "\u{1F9D0}".to_owned(),
        "frowning" => "\u{2639}".to_owned(),
        _ => "\u{1F47B}".to_owned()
    }
}

fn optional_string(x: &Option<&str>) -> String {
    match x {
        Some(ref a) => a.to_string(),
        None => "no value".to_string()
    }
}

fn optional_struct(x: &Tablet) -> String {
    match x.name {
        ref value => format!("The name is: {}", value)
    }
}

fn main() {
    let x = "monocle".to_owned();
    println!("{:?}", string_value(&x));

    let data = Some("\u{1F925}");
    // let data = Some(&2);
    println!("{:?}", optional_string(&data));

    pointer_arguments(&12, &-11_32);

    let tab = Tablet {
        name: String::from("Aspirin"),
        boiling_point: 140.2
    };

    let boiling_point = optional_struct(&tab);

    println!("{:?}", boiling_point);

    println!("{:?}", pointer_reference());
}