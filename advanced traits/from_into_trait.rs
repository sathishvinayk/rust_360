use std::net::{Ipv4Addr};
fn into_example<T: Into<Vec<u8>>>(x: T) {
    println!("Value of x after calling into {:?}", x.into());
}

fn ipv<R: Into<Ipv4Addr>>(addr: R) {
    let ip_address = addr.into();
    
    println!("Is private {}", ip_address.is_private());
}

fn main() {
    into_example("bob".to_string());

    ipv(Ipv4Addr::new(10, 0, 0, 1));
    ipv([10, 0, 0, 1]);
    ipv(0xd076eb94_u32);

    ipv(Ipv4Addr::from([10, 0, 0, 1]));
    ipv(Ipv4Addr::from(0xd076eb94_u32));
}