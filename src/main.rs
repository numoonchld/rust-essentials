mod a_hello_world;
mod b_primitive_data_types;

fn main() {
    println!("=========================================");
    println!("PRIMITIVE VARIABLES");
    // primitive data types
    b_primitive_data_types::primitive_data_types();
    println!("Integers --------------------------------");
    b_primitive_data_types::integers();
    println!("Floats --------------------------------");
    b_primitive_data_types::floating_points();
    println!("Arithmetic --------------------------------");
    b_primitive_data_types::arithmetic_operations();

    println!("=========================================");
    println!("HELLO WORLD");
    // anatomy of a program
    a_hello_world::hello_world();
}
