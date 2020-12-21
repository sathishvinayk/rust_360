fn find_day(x: &str) {
    match x {
        "monday" => println!("Lets begin work!"),
        _ => println!("Off work")
    }
}

fn find_month<'lifetimeb>(x: &'lifetimeb str) -> &'lifetimeb str {
    match x {
        "july" => &"Its july month", 
        _ => &"not a july"
    }
}

fn main() {
    find_day(&"monday");
    {
        let month = find_month(&"december");
    }
    println!(":=> {}", month); // Error cannot find month in this scope
}