trait Output {
    fn output(&self);
}

struct Block {
    number: Box<i32>
}

struct Station {
    origin: String,
    destination: String
}

trait Locomotive {
    fn travel(&self) -> String;
}

impl Output for Block {
    fn output(&self) {
        println!("{:p}: {:?}", self.number, self.number);
    }
}

impl Locomotive for Station {
    fn travel(&self) -> String {
        format!("The locomotive departs from {} to {}", self.origin, self.destination)
    }
}

fn print_output(block: impl Output) {
    block.output()
}

fn print_output_2(block: &impl Output) {
    block.output()
}

fn transport(station: impl Locomotive) -> impl Locomotive {
    println!("{}", station.travel());
    Station {
        origin: String::from("Manchestor"),
        destination: String::from("Liverpool")
    }
}

fn main() {
    let block = Block {
        number: Box::new(1)
    };
    print_output_2(&block);
    print_output(block);

    let station = Station {
        origin: String::from("Liverpool"),
        destination: String::from("Manchestor")
    };

    let value = transport(station);
    println!("{}", value.travel());
}