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
