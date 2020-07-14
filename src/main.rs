mod pattern_matching;

use pattern_matching::Shape;

fn main() {
    let a = 5;
    let location = Location {
        latitude: 10.0,
        longitude: 20.0,
    };
    print(location);

    let [first, second] = ['A', 'B'];
    println!("First: {}, Second:{}", first, second);

    let [first, .., last] = [1, 2, 3, 4];
    println!("First: {}, last: {}", first, last);

    let shape = Shape::Rectangle(5.0, 8.0);
    // let Shape::Rectangle(height, width) = shape;
    println!("Area of reactangle is {}", shape.area())
}

fn print(location: Location) {
    let Location {
        latitude,
        longitude,
    } = location;
    println!("Latitude: {}, Longitude: {}", latitude, longitude);
}

struct Location {
    latitude: f32,
    longitude: f32,
}
