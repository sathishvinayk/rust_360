use std::collections::BTreeSet;

macro_rules! repeat_set {
    ($x:ty, $($a: expr),* ; $y: ident ) => { 
        {
            let mut data = <$x>::new();
            $(
                data.$y($a);
            )*

            data
        }
    };
}

fn main() {
    let value = repeat_set!(BTreeSet<&str>, "a", "b", "c"; insert);
    println!("{:?}", value);
}