use std::cmp::{Ordering, PartialOrd};

fn float_cmp(x: &&f64, y: &&f64) -> Ordering {
    x.partial_cmp(y).unwrap()
}

fn min_max_cmp() {
    let random_numbers: Vec<u16> = vec![14, 512, 63, 75, 423];

    assert_eq!(random_numbers.iter().max(), Some(&512));
    assert_eq!(random_numbers.iter().min(), Some(&14));
}

fn min_max_by_cmp() {
    let floats = vec![1.2, 5.3, 6.1];

    assert_eq!(floats.iter().min_by(float_cmp), Some(&1.2));
    assert_eq!(floats.iter().max_by(float_cmp), Some(&6.1));
}

fn count_e() {
    assert_eq!(vec![1,2,3,4].iter().count(), 4);
}

fn computation() {
    assert_eq!((1..5).sum::<u8>(), 10);
    assert_eq!(vec![1,2,3,4,5].iter().product::<u8>(), 120);

    assert_eq!(vec![1,2,3,4].iter().nth(2), Some(&3));
    assert_eq!(vec![1,2,3,4].iter().nth(0), Some(&1));
    assert_eq!(vec![1,2,3,4].iter().last(), Some(&4));

    assert_eq!("junk".chars().position(|x| x == 'n'), Some(2));
    assert_eq!((1..9).rposition(|x| x == 3), Some(2));
    assert_eq!(vec![1,2,3,4,5].iter().fold(0, |a, b| a + b ), 15);
    assert_eq!(["The ", "rust"].iter().fold(String::new(), |mut a, &b| {
        a.push_str(b);
        a
    }), "The rust");
}

fn main() {
    min_max_cmp();
    min_max_by_cmp();
    count_e();
    computation();
}