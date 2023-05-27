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
