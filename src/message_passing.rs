use std::sync::mpsc;
use std::thread;

pub fn run() {
    let mut threads = vec![];
    let mut counter = 0;
    let (sender, receiver) = mpsc::channel();
    for i in 1..=10 {
        let tx = sender.clone();
        threads.push(thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));
            tx.send((i, 1)).unwrap();
        }));
    }

    for _ in 1..=threads.len() {
        let (thread, increment) = receiver.recv().unwrap();
        counter += increment;
        println!("{} -> {}", thread, counter);
    }

    for t in threads {
        t.join().unwrap();
    }
}
