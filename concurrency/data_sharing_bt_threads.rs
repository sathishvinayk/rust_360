use std::sync::Arc;

fn loop_value(a: Arc<Vec<usize>>) {
    for value in 0..4 {

        let y = a.clone();
        std::thread::spawn(move || {
            println!("looping thru each value {}", y[value]);
        }).join().expect("Error while spawning!");
    }
}
fn main() {
    let x = Arc::new(vec![1,2,3,4,5]);

    loop_value(x);
}