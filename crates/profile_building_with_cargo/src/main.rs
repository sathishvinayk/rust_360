#[derive(Debug)]
struct Axis {
    x: f32
}

mod tests {
    #[test]
    fn test1() {
        let version: u8 = 12;

        #[cfg(debug_assertions)]
        assert_eq!(version, 12);

        #[cfg(not(debug_assertions))]
        assert_eq!(version, 7);
    }    
}

fn main() {
    let _axis = Axis {x: 3.0 };

    // println!("{:?}", axis);

    // panic!("this is a terrible mistake!");
}