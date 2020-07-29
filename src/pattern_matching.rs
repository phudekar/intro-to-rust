use std::f64::consts::PI;

pub fn run() {
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

pub enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    pub fn area(&self) -> f64 {
        Shape::rounded(match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(height, width) => height * width,
            Shape::Triangle(a, b, c) => {
                let perimeter = (a + b + c) / 2.0;
                (perimeter * (perimeter - a) * (perimeter - b) * (perimeter - c)).sqrt()
            }
        })
    }

    fn rounded(n: f64) -> f64 {
        (n * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern_matching::Shape;
    #[test]
    fn should_calculate_area() {
        assert_eq!(Shape::Circle(2.0).area(), 12.57);
        assert_eq!(Shape::Square(5.0).area(), 25.0);
        assert_eq!(Shape::Rectangle(5.0, 3.0).area(), 15.0);
        assert_eq!(Shape::Triangle(24.0, 30.0, 18.0).area(), 216.0);
    }
}
