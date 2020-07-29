mod btree;
mod complex_types;
mod generics;
mod library;
mod message_passing;
mod ownership;
mod pattern_matching;
mod rc;
mod threads;
mod transport;
mod unique_words;

fn main() {
    complex_types::run();
    pattern_matching::run();
    ownership::run();
    transport::run();
    generics::run();
    btree::run();
    rc::run();
    threads::run();
    message_passing::run();
    unique_words::run();
}
