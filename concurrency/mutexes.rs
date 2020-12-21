use std::thread;
use std::sync::{Mutex, Arc};
 
fn mutex_example(x: Mutex<u8>) {
  let vector = Mutex::new(vec![]);

  thread::spawn(move || {
      let y = x.lock().unwrap().clone();
      vector.lock().unwrap().push(y);
  }).join().expect("Error while spawning");
}

fn mut_with_arc() {
  let vector = Arc::new(Mutex::new(vec![]));
  for x in 0..3 {
       let y = vector.clone();
       thread::spawn(move || {
           let mut y = y.lock().unwrap();
           y.push(x);
       }).join().expect("Error while spawning");
  }
  println!("Vector {:?}", vector);
}

fn main() {
    let x = Mutex::new(5);
  
    mutex_example(x);
    mut_with_arc();
}
 