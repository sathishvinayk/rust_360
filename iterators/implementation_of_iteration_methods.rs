use std::collections::BTreeSet;

fn into_bound<T>(t: T) 
    where 
        T: IntoIterator,
        T::Item : std::fmt::Debug
{
    for x in t {
        println!("Value of x: {:?}", x);
    }
}

fn into_iter_example(){
    let mut btree = BTreeSet::new();

    btree.insert("circle".to_string());
    btree.insert("square".to_string());

    let mut ii = btree.into_iter();
    println!("The next value from into_iter: {:?}", ii.next());
    // println!("The next value from into_iter: {:?}", ii.next());
    assert_eq!(ii.next(), Some("square".to_string()));

    let mut btree2 = BTreeSet::new();
    btree2.insert(&"penta");
    btree2.insert(& "hexa");

    let mut ii2 = btree2.into_iter();
    assert_eq!(ii2.next(), Some(&"hexa"));

    let mut vector = vec![1,2,3,4,5];

    let mut iterator = (&mut vector).into_iter();
    *iterator.next().unwrap() += 5;

    println!("New vector {:?}", vector);
}

fn iter_mut_example() {
    let mut vector = vec![1,2,3,4,5];

    let mut iterator = vector.iter_mut();
    *iterator.next().unwrap() += 5;

    println!("New vector {:?}", vector);
}

fn iter_example() {
    let vector = vec![1,2,3,4,5];

    let mut iterator = vector.iter();

    println!("The next value: {:?}", iterator.next());

    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&3));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&5));
    println!("The last value: {:?}", iterator.next());
    println!("The last value: {:?}", iterator.next());
    println!("The last value: {:?}", iterator.next());
}

fn drain_method() {
    let mut vector = vec![1,2,3,4,5];
    {
        let mut _drain_it = vector.drain(..);
    }

    println!("Vector from drained method: {:?}", vector);
}

fn main() {
    iter_example();
    iter_mut_example();

    into_iter_example();

    into_bound(vec![1,2,3]);
    drain_method();
}
