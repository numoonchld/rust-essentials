pub fn scope() {
    let planet = "Earth";
    println!("planet is {planet}");

    if true {
        let planet_2 = "Mars";
        println!("planet is {planet_2}");
    }

    // planet_2 is out of scope here!
    // but planet is in scope outside and inside the curly braces {}

    let planet = 6;
    println!("planet is number {planet}");
}

pub fn shadow_and_scope() {
    let planet = "Earth";
    {
        println!("planet is {planet}");
        let planet = "Mars";
        println!("planet is {planet}");
    }
    println!("planet is {planet}");
}

pub fn string_type() {
    let mut message = String::from("Earth");
    println!("message is: {message}");

    message.push_str(" is home"); // this grows the string on the heap to include the appended string
    println!("message is: {message}");
}

pub fn ownership_move() {
    // shallow copy
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("inner_planet is {inner_planet}");
        outer_planet = inner_planet;
        // here on, inner_planet is no longer a valid variable, since the ownership of the value it held has been transferred and the variable is thrown out of memory
    }
    println!("outer_planet is {outer_planet}");
}

pub fn ownership_clone() {
    // deep copy => two separate instance of the value
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("inner_planet is {inner_planet}");
        outer_planet = inner_planet.clone();
        println!("inner_planet is {inner_planet}");
    }
    println!("outer_planet is {outer_planet}");
}

pub fn ownership_copy() {
    // integer values are cloned by default instead of ownership transfer and variable invalidation since they live on the stack
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        println!("inner_planet is {inner_planet}");
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {inner_planet}");
    }
    println!("outer_planet is {outer_planet}");
}

pub fn ownership_transfer_in_functions_stack_resource() {
    let rocket_fuel = 1;
    process_fuel(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");
}

fn process_fuel(mut propellant: i32) {
    propellant += 1;
    println!("processing propellant: {propellant}");
}

pub fn ownership_transfer_in_functions_heap_resource() {
    let rocket_fuel = String::from("RP-1");
    process_fuel_heap(rocket_fuel);
    // since string is stored on the heap, rocket_fuel will be invalid here onwards
}

pub fn ownership_transfer_in_functions_heap_resource_clone() {
    let rocket_fuel = String::from("RP-1");
    process_fuel_heap(rocket_fuel.clone());
    println!("rocket_fuel is {rocket_fuel}");
}

fn process_fuel_heap(propellant: String) {
    println!("processing propellant: {}", propellant);
}

pub fn ownership_transfer_in_functions_heap_resource_return_ownership() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel_heap_return(rocket_fuel);
    println!("rocket_fuel is {rocket_fuel}");
}

fn process_fuel_heap_return(propellant: String) -> String {
    println!("processing propellant: {}", propellant);
    propellant
}
