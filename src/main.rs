fn main() {
    let lion = Lion {};
    lion.make_sound();

    let dog = Dog {};
    dog.make_sound();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Lion {}), Box::new(Dog {})];

    animals.iter().for_each(|animal| animal.make_sound());
}

trait Animal {
    fn make_sound(&self);
}

struct Lion;

impl Animal for Lion {
    fn make_sound(&self) {
        println!("Lion roars!")
    }
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Dog barks!")
    }
}
