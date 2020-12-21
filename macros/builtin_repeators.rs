macro_rules! vcx {
    () => (
        ::std::vec::Vec::<u8>::new()
    );
    ($elem:expr; $n:expr) => (
        ::std::vec::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(Box::new([$($x),+]) )
    )
 }
 
 
 fn main() {
     let vector = vcx![1,2,3];
 
     let vector2 = vcx![2; 4];
 
     let vector3 = vcx![];
 
     println!("{:?}", vector);
     println!("{:?}", vector2);
     println!("{:?}", vector3);
 }
 
 