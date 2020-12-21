use std::collections::HashMap;
use std::net::*;

use std::collections::HashSet as SETID;

fn main() {
    let mut hash_1 = ::std::collections::HashMap::<u8, u8>::new();
    let mut hash_2: HashMap<u8, u8> = HashMap::new();

    let _value = TcpStream::connect("127.0.0.1");

    let mut set_new_id = SETID::new();

    set_new_id.insert("ASFA_ASD");

    assert_eq!(set_new_id.contains("ASFA_ASD"), true);

    hash_1.insert(1, 12);
    hash_1.insert(2, 14);

    hash_2.insert(5, 51);

    assert_eq!(hash_1[&1], 12);
    assert_eq!(hash_1[&2], 14);

    assert_eq!(hash_2[&5], 51);
}