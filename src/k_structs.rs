#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

pub fn structs_demo() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 836746.0,
    };
    vehicle.name = String::from("Atlantis");
    println!("Vehicle is {:?}", vehicle);

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    vehicle.crew_size = 5;

    println!("Vehicle is {:?}", vehicle);
    println!("Vehicle 2 is {:?}", vehicle2);
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

pub fn struct_methods() {
    let mut vehicle = Shuttle::new("Endeavor");
    let mut vehicle2 = Shuttle::new("Discovery");

    // let mut vehicle = Shuttle {
    //     name: String::from("Endeavour"),
    //     crew_size: 7,
    //     propellant: 0.0,
    // };

    let vehicle_name = vehicle.get_name();
    println!("Vehicle name is {}", vehicle_name);

    println!("propellant value is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant value is {}", vehicle.propellant);
}

struct Color(u8, u8, u8); //RGB
struct Point(u8, u8, u8); //XYZ-point

fn get_y(p: Point) -> u8 {
    p.1
}

pub fn tuple_structs() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y);
}
