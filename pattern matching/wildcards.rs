#[allow(dead_code)]
enum Mammal {
    Cat,
    Panda,
    Hamster
}

#[allow(dead_code)]
#[derive(Debug)]
struct Power {
    current_type: (String, u8, i8),
    wave: String
}

fn display_id(id: u8, _: String) -> String {
    format!("Returning only the id: {}", id)
}

fn main() {
    let mammal = Mammal::Panda;

    let voice = match mammal {
        Mammal::Cat => "Meow!",
        _ => "No voice found in the server!",
        // Mammal::Panda => "Growllll!"
    };

    let _power = Power {
        current_type: ("Direct".to_owned(), 5, -2),
        wave: "Sin".to_owned()
    };

    let value = match _power {
        Power {
            current_type: tt,
            wave: _
        } => tt
    };

    let _power_2 = Power {
        current_type: ("Direct".to_owned(), 5, -2),
        wave: "cos".to_owned()
    };

    let Power {
        current_type: (_, positive, _),
        wave: _
    } = _power_2;

    let (a, b, _, d) = (1,2,3,4);

    let name = Some(String::from("Adam"));
    match name {
        Some(_) => println!("Access granted for all"),
        None => println!("No access!")
    }

    let _value_2 = "Bob".to_string();

    let (first, ..) = (1,2,3,4,5);
    let (first_1, .., last_1) = (1,2,3,4,5);

    println!("{}", display_id(12, _value_2));

    println!("Voice of mammal: {}", voice);
    println!("Value of the power: {:?}", value);

    println!("Value of positive is: {:?}", positive);
    println!("Values of a, b, d is {} {} {}", a, b, d);
    println!("First value is {}", first);
    println!("First and last value is {}, {}", first_1, last_1);
}