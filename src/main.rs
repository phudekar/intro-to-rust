mod sahaj;

use sahaj::pattern_matching::Shape;

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
    println!("Area of rectangle is {}", shape.area())
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
