use std::thread;

fn main() {
    let t0 = thread::spawn(|| {
        println!("This is running on a seperate thread!")
    });
    t0.join().expect("Error occured!");

    let x = thread::Builder::new().name("t1".into());

    let y = x.spawn(|| {
        println!("Current thread: {:?}", thread::current().name())
    }).unwrap();

    y.join().unwrap();
}