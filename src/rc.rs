use std::cell::Cell;
use std::rc::Rc;

pub fn run() {
    let alex = Rc::new(Person::new("Alex", 111111111));
    let mut college = College::new();
    college.add_student(Rc::clone(&alex));

    let mut office = Office::new();
    office.add_employee(Rc::clone(&alex));

    college.contact();
    office.contact();

    alex.change_contact_number(2222222);

    college.contact();
    office.contact();
}

#[derive(Clone)]
struct Person {
    name: String,
    contact_number: Cell<u32>,
}

impl Person {
    pub fn new(name: &str, contact_number: u32) -> Self {
        Person {
            name: String::from(name),
            contact_number: Cell::new(contact_number),
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn contact_number(&self) -> u32 {
        self.contact_number.get()
    }

    pub fn change_contact_number(&self, new_contact_number: u32) {
        self.contact_number.set(new_contact_number);
    }
}

struct College {
    students: Vec<Rc<Person>>,
}

impl College {
    pub fn new() -> Self {
        College { students: vec![] }
    }
    pub fn add_student(&mut self, student: Rc<Person>) {
        self.students.push(student)
    }
    pub fn contact(&self) {
        for person in self.students.iter() {
            println!(
                "College contacting {} on {}",
                person.name(),
                person.contact_number()
            );
        }
    }
}

struct Office {
    employees: Vec<Rc<Person>>,
}

impl Office {
    pub fn new() -> Self {
        Office { employees: vec![] }
    }
    pub fn add_employee(&mut self, employee: Rc<Person>) {
        self.employees.push(employee)
    }
    pub fn contact(&self) {
        for person in self.employees.iter() {
            println!(
                "Office contacting {} on {}",
                person.name(),
                person.contact_number()
            );
        }
    }
}
