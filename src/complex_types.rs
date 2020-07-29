use crate::library::{Book, Library};

pub fn run() {
    let mut library = Library::new();
    let title = "Introduction to Rust";
    let book = Book::new(title, "Community", 10.0);

    library.add_book(book);
    library.add_book(Book::new("Alice in Wonderland", "Lewis Carroll", 15.0));
    println!(
        "Book available: {}",
        if library.is_book_available(title) {
            "Yes"
        } else {
            "No"
        }
    );

    let ferris = Person::new_student("Ferris", 15, "FR_1");
    borrow(&mut library, title, &ferris);

    let bob = Person::new("Bob", 24);
    borrow(&mut library, title, &bob);
    borrow(&mut library, "Alice in Wonderland", &bob);
}

fn borrow(library: &mut Library, title: &str, person: &Person) {
    match library.borrow(title, person.clone()) {
        Ok((book, cost)) => {
            let currency = '₹';
            println!(
                "{} is borrowed by {} for {}{}",
                book.title, person.name, currency, cost
            )
        }
        Err(error) => println!("{}", error),
    }
}

#[derive(Clone, Debug)]
pub struct Person {
    pub name: String,
    age: u8,
    pub member_type: Member,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: String::from(name),
            age,
            member_type: Member::Visitor,
        }
    }

    pub fn new_student(name: &str, age: u8, student_id: &str) -> Person {
        Person {
            name: String::from(name),
            age,
            member_type: Member::Student(String::from(student_id)),
        }
    }

    pub fn is_teen(&self) -> bool {
        self.age < 18
    }
}

#[derive(Clone, Debug)]
pub enum Member {
    Student(String),
    Visitor,
}
