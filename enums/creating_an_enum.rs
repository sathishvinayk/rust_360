#![allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South = 99,
    East,
    West
}

#[derive(Debug)]
enum Novariant {

}
impl Novariant {
    fn new() {
        println!("Hi this is offset function!");
    }
}

impl Direction {
    fn find(self) -> &'static str {
        match self {
            Direction::North => "North pole",
            Direction::South => "South pole",
            Direction::East => "East pole",
            Direction::West => "West pole",
        }
    }
}

fn find_direction(x: Direction) {
    if x == Direction::North {
        println!("Direction is north!");
    } else {
        println!("Direction is not north side!");
    }
}

fn main() {
    let value: Direction = Direction::South;

    println!("The direction is {:?}", value);

    find_direction(Direction::North);

    let south = Direction::South as u32;
    let north = Direction::North as u32;

    println!("South value is {:?}", south);
    println!("North value is {:?}", north);

    let west = Direction::West as u32;
    let east = Direction::East as u32;

    println!("West value is {:?}", west);
    println!("East value is {:?}", east);

    assert_eq!(value.find(), "South pole");

    let _no_variant = Novariant::new();
}