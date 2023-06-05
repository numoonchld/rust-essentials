pub fn no_borrowing() {
    let rocket_fuel = String::from("RP-1");
    let (rocket_fuel, length) = process_fuel(rocket_fuel);
    println!(
        "rocket_fuel is {} and the length is {}",
        rocket_fuel, length
    )
}

fn process_fuel(propellant: String) -> (String, usize) {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    (propellant, length)
}

pub fn borrowing_for_access() {
    let rocket_fuel = String::from("RP-1");
    let length = process_fuel_with_borrowing_for_access(&rocket_fuel);
    println!(
        "rocket_fuel is {} and the length is {}",
        rocket_fuel, length
    )
}

fn process_fuel_with_borrowing_for_access(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    length
}

pub fn borrowing_for_modification() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel_with_borrowing_for_modification(&mut rocket_fuel);
    println!(
        "rocket_fuel is {} and the length is {}",
        rocket_fuel, length
    )
}

fn process_fuel_with_borrowing_for_modification(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable");
    let length = propellant.len();
    length
}

pub fn sliced_referencing() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word = &message[15..15 + 5]; // the units are bytest
    println!("last word is {} ", last_word);
}
