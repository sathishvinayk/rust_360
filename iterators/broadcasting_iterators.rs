use std::str::FromStr;
use std::collections::HashMap;

fn frmap() {
    let x = vec!["1","2","3","4","5"];
    let data = x.iter();
    
    let mut x = data.filter_map(|a| u8::from_str(a).ok());

    assert_eq!(x.next(), Some(1));
}

fn without_flatmap() {
    let x = vec!["1","2","3","4","5"];
    let data = x.iter();
    
    let mut x = data.map(|n| u8::from_str(n)).filter(|c| c.is_ok()).map(|t| t.unwrap());

    assert_eq!(x.next(), Some(1));
}

fn flat_m() {
    let mut x = HashMap::new();
    x.insert("1", vec!["igloo", "tower", "block"]);
    x.insert("2", vec!["ford", "fierra", "kia"]);
    x.insert("3", vec!["arch", "beam", "narrow"]);

    let keys = ["1", "2"];

    for &bridge in keys.iter().flat_map(|data| &x[data]) {
        println!("Bridge is {}", bridge);
    }
}

fn scan_it() {
    let vector = vec![1,2,3,4,5];
    let mut x = vector.iter().scan(10, |state, next| {
        *state = *state * next;
        Some(*state)
    });

    println!("X: {:?}", x.next());
    println!("X: {:?}", x.next());
    println!("X: {:?}", x.next());
    println!("X: {:?}", x.next());
    println!("X: {:?}", x.next());
    println!("X: {:?}", x.next());

    let y: String = "Per favore".chars().inspect(|x| println!("Inspecting the characters {}", x))
    .flat_map(|i| i.to_lowercase()).chain("abc".chars().rev()).collect();

    println!("Value of y is {}", y);
}

fn nexback() {
    let vector = vec!["vietnam", "cuba", "haiti"];
    
    let mut countries = vector.iter();
    assert_eq!(countries.next_back(), Some(&"haiti"));
    assert_eq!(countries.next(), Some(&"vietnam"));
    assert_eq!(countries.next_back(), Some(&"cuba"));
    assert_eq!(countries.next_back(), None);
    assert_eq!(countries.next(), None);
}

fn enumerator() {
    let vector = vec!['a', 'b', 'c'];
    let second_vector = vec!['d', 'e', 'f'];
    for (i, value) in vector.into_iter().zip(second_vector).enumerate() {
        println!("Index number is {} and value is {:?}", i, value);
    }
}

fn fuse() {
    let vector = vec![1,2,3,4,5];
    let mut iterator = vector.iter();
    
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());

    let mut iterator = iterator.fuse();
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
}

fn by_reference() {
    let text = "this one\r\n
        is just a sample text\r\n
        with some junk sentence\r\n";

    let mut lines = text.lines();
    for line in lines.by_ref().take_while(|x| !x.is_empty()) {
        println!("Line {}", line);
    }
    for line in lines.by_ref().skip(1).skip_while(|x| !x.is_empty()) {
        println!("Line {}", line);
    }
}

fn clond_example() {
    let a = ['a', 'b', 'c'];
    let mut iteration = a.iter();
    let mut clone_iteration = a.iter().cloned();
    
    assert_eq!(iteration.next(), Some(&'a'));
    assert_eq!(clone_iteration.next(), Some('a'));
}

fn cycling() {
    let vector = vec!["direct", "inverse", "joint"];
    let mut cycle_it = vector.iter().cycle();

    assert_eq!(cycle_it.next(), Some(&"direct"));
    assert_eq!(cycle_it.next(), Some(&"inverse"));
    assert_eq!(cycle_it.next(), Some(&"joint"));
    assert_eq!(cycle_it.next(), Some(&"direct"));
    assert_eq!(cycle_it.next(), Some(&"inverse"));
}

fn map_filter() {
    let mpx = vec![1, 2, 3, 4, 5];
    let data = mpx.into_iter();
    let x: Option<u8> = data.map(|n| {
        n + 1
    }).filter(|x| x != &2 ).next();

    println!(" x {:?}", x);
}

fn main() {
    map_filter();
    frmap();
    without_flatmap();
    flat_m();
    scan_it();
    nexback();
    enumerator();
    fuse();
    by_reference();
    clond_example();
    cycling();
}