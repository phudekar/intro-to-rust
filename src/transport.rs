use geoutils::Location;

pub trait ModeOfTransport {
    fn charges_per_km(&self) -> f64;
}

pub struct Bus {
    route: u32,
}

impl ModeOfTransport for Bus {
    fn charges_per_km(&self) -> f64 {
        5.0
    }
}

#[derive(Debug)]
pub enum MetroLine {
    Red,
    Blue,
    Gree,
}

pub struct Metro {
    pub line: MetroLine,
}

impl ModeOfTransport for Metro {
    fn charges_per_km(&self) -> f64 {
        10.0
    }
}

trait Distance {
    fn get_distance_in_km(&self, other: &Self) -> f64;
}

#[derive(Clone)]
struct Station {
    pub name: &'static str,
    location: Location,
}

impl Distance for Station {
    fn get_distance_in_km(&self, other: &Self) -> f64 {
        if let Ok(distance) = self.location.distance_to(&other.location) {
            distance.meters() / 1000.0
        } else {
            0.0
        }
    }
}

struct Stations {
    all_stations: Vec<Station>,
}

impl Stations {
    pub fn stations_in_pune() -> Stations {
        Stations {
            all_stations: vec![
                Station {
                    name: "Swargate",
                    location: Location::new(18.5018, 73.8636),
                },
                Station {
                    name: "Viman Nagar",
                    location: Location::new(18.5679, 73.9143),
                },
            ],
        }
    }
    pub fn find(&self, name: &str) -> Option<&Station> {
        self.all_stations
            .iter()
            .find(|station| station.name == name)
    }
}

pub struct Journey {
    from: Station,
    to: Station,
}

impl Journey {
    pub fn new(from: &str, to: &str) -> Journey {
        let stations = Stations::stations_in_pune();
        let from = stations.find(from).expect("Expected from station");
        let to = stations.find(to).expect("Expected to station");
        Journey {
            from: from.clone(),
            to: to.clone(),
        }
    }

    pub fn fare<T>(&self, mode: &T) -> f64
    where
        T: ModeOfTransport,
    {
        (self.from.get_distance_in_km(&self.to) * mode.charges_per_km()).round()
    }
}

#[cfg(test)]
mod test {
    use crate::transport::{Bus, Journey, Metro, MetroLine};

    #[test]
    fn should_calculate_fare_for_journey_by_bus() {
        let journey = Journey::new("Swargate", "Viman Nagar");
        let bus = Bus { route: 14 };
        assert_eq!(45.0, journey.fare(&bus))
    }

    #[test]
    fn should_calculate_fare_for_journey_by_metro() {
        let journey = Journey::new("Swargate", "Viman Nagar");
        let metro = Metro {
            line: MetroLine::Red,
        };
        assert_eq!(91.0, journey.fare(&metro))
    }
}
