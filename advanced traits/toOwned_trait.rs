fn clone_example() {
    let s1: &'static str = "I am static";
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1.clone();
    let c2 = s2.clone();

    assert_eq!(c1, s1);
    assert_eq!(c2, s2);
}

fn toowned_example() {
    let s1: &'static str = "I am static";
    let s2 = "I am boxed and owned".to_string();

    let c1 = s1.to_owned();
    let c2 = s2.to_owned();

    assert_eq!(c1, s1);
    assert_eq!(c2, s2);
}

fn main() {
    clone_example();
    toowned_example();
}