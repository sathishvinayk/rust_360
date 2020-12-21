macro_rules! check_not_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!(r#"assertion failed: `(left != right)` left: `{:?}`, right: `{:?}`"#, 
                    &*left_val, &*right_val)
                }
            }
        }
    });
}

fn main() {
    let validate = check_not_eq!(1, 1);

    println!("Validated value is {:?}", validate);
}

