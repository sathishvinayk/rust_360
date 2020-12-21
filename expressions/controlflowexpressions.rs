enum Package {
    Data,
    Err
}
fn enum_let() {
    let output = Package::Data;
    let error = Package::Err;

    if let Package::Data = output {
        println!("{}", "Output from Package!")
    }

    if let Package::Err = error {
        println!("{}", "Error 1 appeared!")
    } else {
        println!("{}", "Error 2 appeared !")
    }
}
fn equivalent_match_expression() {
    let value;


    let data = 14;
    match data {
        0 => value = "Value is zero!",
        1 => value = "Value is one!",
        _ => value = "Not initialized"
    }

    println!("{}", value);
}
fn main() {
    let value;

    let data = 0;

    if data == 1 {
        value = "Value is one!"
    } else if data == 0 {
        value = "Value is zero!"
    } else {
        value = "Value is not initialized"
    }

    println!("{}", value);
    equivalent_match_expression();
    enum_let();
}