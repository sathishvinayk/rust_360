use std::sync::mpsc::{Sender, SyncSender, Receiver, channel, sync_channel};

fn sync_example() {
    let (tx, rx): (SyncSender<u8>, Receiver<u8>) = sync_channel(1);

    let _v1 = tx.send(0);
    let send_clone = tx.clone();

    std::thread::spawn(move || {
        let _v2 = tx.send(2);
    });

    std::thread::spawn(move || {
        let _v3 = send_clone.send(3);
    });

    println!("Got sync value {}", rx.recv().unwrap());
    println!("Got sync value {}", rx.recv().unwrap());
    println!("Got sync value {}", rx.recv().unwrap());
}

fn cn_example() {
    let (tx, rx) : (Sender<u8>, Receiver<u8>) = channel();

    let x = std::thread::spawn(move || {
        while let Ok(x) = rx.recv() {
            println!("Got the value {}", x);
        }
    });

    for n in 0..10 {
        tx.send(n).unwrap();
    }

    x.join().expect("Error while spawning!");
}

fn main() {
    // cn_example();
    sync_example();
}