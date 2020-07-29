use library;
pub fn run() {
    let mut library = library::Library::new();
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

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Member {
    Student(String),
    Visitor,
}
