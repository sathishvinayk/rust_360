mod queue;

use queue::*; 
use std::collections::VecDeque;


struct Employee1{
    id: u16,
    level: u8
}

struct Employee2 {
    id: u16,
    level: u8
}

struct Boat<'a> {
    _length: &'a i8
}

trait Badge {
    fn new(name: &'static str, id: u16, level: u8) -> Self;
    fn badge(&self);
    fn finish(&self) {
        println!("Process finished")
    }
}

trait Double {
    fn double_it(self) -> Self;
}

trait Size {}

impl Badge for Employee1 {
    fn new(name: &'static str, id: u16, level: u8) -> Self {
        println!("Printing new badge for the user: {}", name);
        Employee1 {
            id,
            level
        }
    }
    fn badge(&self) {
        if self.level == 2 {
            println!("Badge is blue for id {}", self.id);
        }   
    }
}

impl Badge for Employee2 {
    fn new(name: &'static str, id: u16, level: u8) -> Self {
        println!("Printing new badge for the user: {}", name);
        Employee2 {
            id,
            level
        }
    }
    fn badge(&self) {
        if self.level == 4 {
            println!("Badge is green for the id {}", self.id);
        }
    }
}

impl Double for i8 {
    fn double_it(self) -> Self {
        self * 2
    }
}

impl<'a> Size for Boat<'a> {}

fn main() {
    let emp = Employee1 {
        id: 10014,
        level: 2
    };

    emp.badge();

    let emp2 = Employee2 {
        id: 10045,
        level: 4
    };

    emp2.badge();

    let _boat = Boat {
        _length: &35
    };

    let value = 14.double_it();
    println!("Doubled: {}", value);

    let mut req = API {
        reqs: VecDeque::new()
    };

    req.single_request(5);
    req.multiple_request(&vec![6,7,8]);
    println!("Popped request: {:?}", req.remove_request());

    let name: Employee1 = Badge::new("bob", 1001, 2);
    name.badge();
    name.finish();
}