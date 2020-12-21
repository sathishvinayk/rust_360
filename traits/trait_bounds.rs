use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Random(usize);

#[derive(Debug)]
struct Cog<T>(T);

trait Spin {
    fn kind(&self);
}

impl<T: Debug> Spin for Cog<T> {
    fn kind(&self) {
        println!("The cog kind is {:?}", self.0);
    }
}

fn get_random<T>(value: T) where T: Debug {
    println!("Random value is {:?}", value);
}

fn get_value<U: Display>(value: &U) {
    println!("Reference of random value is {}", value); 
}

fn get_random_2<T>(value: T) -> u8 
    where T:Debug
{
    println!("From random2 value is {:?}", value);
    32
}

fn kind<T: Spin>(cog: T) {
    cog.kind()
}

fn main() {
    let random = Random(2);
    get_value(&random.0);
    // get_random(random);
    get_random_2(random);

    let cog = Cog("Sprocket");
    kind(cog);
}