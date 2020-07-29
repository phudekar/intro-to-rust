pub fn run() {
    let mut age = 7;
    age = 8;

    let teenager = is_teenager(age);
    println!("Teenager: {}", teenager);
    println!("Age is {}", age);

    let mut name = String::from("Ferris");
    let length = length(&name);
    println!("Length is {}", length);
    println!("Name is {}", name);
    greet(&mut name);
    println!("{}", name);

    // let name = String::from("Ferris");
    // let length = length_with_borrow(&name);
    // println!("Name after borrow is {}", name);
}

fn is_teenager(age: i32) -> bool {
    age > 12 && age < 18
}

fn greet(name: &mut String) {
    name.push('H');
}

fn length(str: &String) -> usize {
    str.len()
}

fn length_with_borrow(str: &String) -> usize {
    str.len()
}
