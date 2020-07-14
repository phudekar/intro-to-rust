mod complex_types;
mod library;

use complex_types::Person;
use library::Library;

fn main() {
    let mut library = Library::new();
    let title = "Introduction to Rust";
    let book = library::Book::new(title, "Community", 10.0);
    library.add_book(book);
    println!(
        "Book available: {}",
        if library.is_book_available(title) {
            "Yes"
        } else {
            "No"
        }
    );

    let person = Person::new_student("Ferris", 15, "FR_1");
    let (book, cost) = library
        .borrow(title, person.clone())
        .expect("Expected book after borrowing");
    let currency = '₹';
    println!(
        "{} is borrowed by {} for {}{}",
        book.title, person.name, currency, cost
    )
}
