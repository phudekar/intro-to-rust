mod complex_types;

use complex_types::Person;

fn main() {
    let name = "Ferris";
    let age = 7;

    let ferris = Person::new(name, age);

    if ferris.is_teen() {
        println!("{} can vote", ferris.name);
    } else {
        println!("{} is not eligible to vote", ferris.name);
    }
}
