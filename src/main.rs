mod threads;
use std::thread;

fn main() {
    let mut threads = vec![];
    let mut count = 0;
    for i in 1..=10 {
        threads.push(thread::spawn(move || {
            count += 1;
            thread::sleep(std::time::Duration::from_secs(1));
            println!("{} -> {}", i, count)
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    // threads::run();
}
