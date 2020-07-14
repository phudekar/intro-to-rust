use crate::complex_types::{Member, Person};

pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn is_book_available(&self, title: &str) -> bool {
        self.books.iter().filter(|book| book.title == title).count() > 0
    }

    pub fn borrow(&mut self, title: &str, person: Person) -> Result<(Book, f32), &str> {
        if let Some(book) = self.books.iter_mut().find(|book| book.title == title) {
            book.lend_to(&person);
            let cost = book.price - book.price * person.get_discount();
            Ok((book.clone(), cost))
        } else {
            Err("Book is not available!")
        }
    }
}

#[derive(Clone)]
pub struct Book {
    pub title: String,
    author: String,
    borrowed_by: Option<Person>,
    price: f32,
}

impl Book {
    pub fn new(title: &str, author: &str, price: f32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            borrowed_by: None,
            price,
        }
    }

    fn lend_to(&mut self, person: &Person) {
        self.borrowed_by = Some(person.clone())
    }
}

trait Pricing {
    fn get_discount(&self) -> f32;
}

impl Pricing for Person {
    fn get_discount(&self) -> f32 {
        match self.member_type {
            Member::Student(_) => 0.5,
            Member::Visitor => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::complex_types::Person;
    use crate::library::{Book, Library, Pricing};

    #[test]
    fn should_return_false_if_book_not_available() {
        let library = Library::new();
        let is_available = library.is_book_available("Introduction to Rust");
        assert_eq!(false, is_available);
    }

    #[test]
    fn should_return_true_if_book_not_available() {
        let mut library = Library::new();
        library.add_book(Book::new("Introduction to Rust", "Community", 0.0));
        let is_available = library.is_book_available("Introduction to Rust");
        assert_eq!(true, is_available);
    }

    #[test]
    fn should_return_discount_for_student() {
        let student = Person::new_student("Ferris", 15, "FR_1");
        assert_eq!(0.5, student.get_discount())
    }

    #[test]
    fn should_return_discount_for_visitor() {
        let visitor = Person::new("Ferris", 15);
        assert_eq!(0.0, visitor.get_discount())
    }

    #[test]
    fn should_return_book_with_cost_of_borrowing() {
        let mut library = Library::new();
        library.add_book(Book::new("Introduction to Rust", "Community", 10.0));
        let person = Person::new("Ferris", 15);
        let (book, cost) = library
            .borrow("Introduction to Rust", person)
            .expect("Expected book after borrowing");
        assert_eq!("Introduction to Rust", book.title);
        assert_eq!(cost, 10.0);
    }

    #[test]
    fn should_return_error_if_book_not_available_while_borrowing() {
        let mut library = Library::new();
        let person = Person::new("Ferris", 15);
        if let Err(error) = library.borrow("Introduction to Rust", person) {
            assert_eq!("Book is not available!", error)
        } else {
            panic!("Should not have returned book")
        }
    }

    #[test]
    fn should_return_book_with_discounted_cost_for_student() {
        let mut library = Library::new();
        library.add_book(Book::new("Introduction to Rust", "Community", 10.0));
        let person = Person::new_student("Ferris", 15, "FR_1");
        let (book, cost) = library
            .borrow("Introduction to Rust", person)
            .expect("Expected book after borrowing");
        assert_eq!("Introduction to Rust", book.title);
        assert_eq!(cost, 5.0);
    }
}
