struct Street {
    number: u8
}

impl Street {
    fn print(&self) {
        let value = |data: bool| if data {self.number} else { 0 }; 

        println!("Street number is {:?}", value(true));
    }
}

fn example() {
    let value = |_| println!("Value is ");

    value(8);
    // value(String::from("Spoiler alert"));
}

fn generate_name(name: char) {
    let names = vec!["Brown", "Blue", "Red"];

    let value: Vec<_> = names.iter().skip_while(|x| x.starts_with(name)).collect();

    println!("The value is {:?}", value[0]);
}

fn main() {
    let name = Box::new("bob");

    let value = || format!("The name is {}", name);

    let street = Street {
        number: 12
    };

    street.print();

    println!("From closure {:?}", value());
    println!("After closure {:?}", name);
    example();

    generate_name('b');
}