use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let mut threads = vec![];
    let counter = Arc::new(Mutex::new(0));
    for i in 1..=10 {
        let count = Arc::clone(&counter);
        threads.push(thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(1));
            if let Ok(mut current) = count.lock() {
                *current += 1;
                println!("{} -> {}", i, *current)
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
