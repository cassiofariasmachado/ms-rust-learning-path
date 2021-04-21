use std::fmt;

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

impl fmt::Display for Transmission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Transmission::Manual => write!(f, "Manual"),
            Transmission::SemiAuto => write!(f, "SemiAuto"),
            Transmission::Automatic => write!(f, "Automatic"),
        }
    }
}

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Color: {}, Transmission: {}, Covertible: {}, Mileage: {}",
            self.color, self.transmission, self.convertible, self.mileage
        )
    }
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    };

    // Factory's Quality Control Department says that new cars must always have zero mileage!
    assert_eq!(car.mileage, 0);
    return car;
}

fn main() {
    let car_1 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(car_1.color, "Red");
    assert_eq!(car_1.transmission, Transmission::Manual);
    assert_eq!(car_1.convertible, false);

    println!("Car 1: {}", car_1);

    let car_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(car_2.color, "Silver");
    assert_eq!(car_2.transmission, Transmission::Automatic);
    assert_eq!(car_2.convertible, true);

    println!("Car 2: {}", car_2);

    let car_3 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(car_3.color, "Yellow");
    assert_eq!(car_3.transmission, Transmission::SemiAuto);
    assert_eq!(car_3.convertible, false);

    println!("Car 3: {}", car_3);
}
