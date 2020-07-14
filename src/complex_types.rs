#[derive(Clone)]
pub struct Person {
    pub name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        Person {
            name: String::from(name),
            age,
        }
    }

    pub fn is_teen(&self) -> bool {
        self.age < 18
    }
}
