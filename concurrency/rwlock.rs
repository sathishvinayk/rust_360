use std::sync::{RwLock, Arc};

fn rw_lock_example() {
    let value: RwLock<u8> = RwLock::new(15); 
    {
        let r1 = value.read().unwrap();
        let r2 = value.read().unwrap();
        let r3 = value.read().unwrap();
        println!("Value of the RwLock is {} {}, {}", r1, r2, r3,);
    }

    {
        let mut r4 = value.write().unwrap();
        *r4 += 2;
    }
    println!("Updated values is {:?}", value);
}

fn rw_with_arc() {
    let value: Arc<RwLock<u8>> = Arc::new(RwLock::new(2));
    let clone_arc = value.clone();

    std::thread::spawn(move || {
        *clone_arc.write().unwrap() += 4;    
    }).join().unwrap();

    println!("Updated rc with arc value is {:?}", value);
}

fn main() {
    rw_lock_example();
    rw_with_arc();
}
