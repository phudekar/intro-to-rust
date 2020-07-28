mod transport;
use transport::{Journey, Metro, MetroLine};

fn main() {
    let from = "Swargate";
    let to = "Viman Nagar";
    let journey = Journey::new(from, to);
    let metro = Metro {
        line: MetroLine::Red,
    };
    println!(
        "Fare from {} to {} by {:?} Line is {}",
        from,
        to,
        MetroLine::Red,
        journey.fare(&metro)
    )
}
