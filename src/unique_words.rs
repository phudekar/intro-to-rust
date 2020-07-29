use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::iter::FromIterator;
use std::sync::mpsc;
use std::thread;
use threadpool::ThreadPool;

pub fn run() {
    let max_workers = 4;
    let pool = ThreadPool::new(max_workers);
    let (sender, receiver) = mpsc::channel();

    let text = fs::read_to_string("alice.txt").unwrap();

    let mut unique_words = HashMap::new();
    let lines = text.lines();
    println!("Total lines: {}", text.lines().count());
    for line in lines {
        let tx = sender.clone();
        let line = String::from(line);
        let thread_number = pool.active_count() + 1;
        pool.execute(move || {
            Worker::new(thread_number + 1, tx).find_unique_words(line);
        });
    }

    loop {
        if let Ok(map) = receiver.recv_timeout(std::time::Duration::from_secs(1)) {
            for (k, v) in map.iter() {
                if unique_words.contains_key(k) {
                    unique_words.insert(k.clone(), unique_words.get(k).unwrap() + v);
                } else {
                    unique_words.insert(k.clone(), v.clone());
                }
            }
        } else {
            break;
        }
    }
    println!("Total unique words: {}", unique_words.len());
    print_top_words(unique_words);
}

fn print_top_words(unique_words: HashMap<String, u32>) {
    let ordered: BTreeMap<_, _> = unique_words
        .iter()
        .filter(|(k, &v)| v >= 100 && k.len() >= 5)
        .collect();
    let mut v = Vec::from_iter(ordered);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    println!("{:#?}", v);
}

struct Worker {
    pub id: usize,
    sender: mpsc::Sender<HashMap<String, u32>>,
}

impl Worker {
    pub fn new(id: usize, sender: mpsc::Sender<HashMap<String, u32>>) -> Self {
        Worker { id, sender }
    }

    pub fn find_unique_words(&self, str: String) {
        let mut map = HashMap::new();
        for word in str.split_whitespace() {
            let w = String::from(
                word.trim()
                    .trim_matches('.')
                    .trim_matches('?')
                    .trim_matches('!')
                    .trim_matches(';')
                    .trim_matches(',')
                    .trim_matches('“')
                    .trim_matches('"')
                    .to_lowercase(),
            );
            if map.contains_key(&w) {
                map.insert(w.clone(), map.get(&w).unwrap() + 1)
            } else {
                map.insert(w, 1)
            };
        }
        self.sender.send(map).unwrap();
    }
}
