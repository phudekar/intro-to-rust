fn main() {
    let age = 7;
    let teenager = is_teenager(age);
    println!("Age is {}", age);

    // let name = String::from("Ferris");
    // let length = length(name);
    // println!("Name is {}", name);

    // let name = String::from("Ferris");
    // let length = length_with_borrow(&name);
    // println!("Name after borrow is {}", name);
}

fn is_teenager(age: i32) -> bool {
    age > 12 && age < 18
}

fn length(str: String) -> usize {
    str.len()
}

fn length_with_borrow(str: &String) -> usize {
    str.len()
}
