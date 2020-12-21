use std::ops::{Deref, DerefMut};
use std::rc::Rc;

struct Picker<T> {
    elements: Vec<T>,
    current: usize
}

impl<T> Deref for Picker<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Picker<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn vector_slice(data: &[u8]) {
    println!("Received data as {:?}", String::from_utf8_lossy(data));
}

fn deref_coercion() {
    let value: Rc<String> = Rc::new(String::from("Sample Junk data"));

    println!("Result is {:?} ", value.find('p'));

    let data = String::from("Again some junk");
    println!("{:?}", data.split_at(4));

    println!("{:?}", String::from("More Junk").as_bytes());

    let vector = vec![77, 111, 114, 101, 32, 74, 117, 110, 107];
    vector_slice(&vector);
}

fn main() {
    let mut dx = Picker {
        elements: vec!['n', 'c', 's', 'e'],
        current: 2
    };

    assert_eq!(*dx, 's');

    *dx = 'r';

    println!(" Updated values: {:?}", dx.elements);
    assert_eq!(dx.is_alphabetic(), true);

    deref_coercion();
}   