mod a_hello_world;
mod b_primitive_data_types;
mod c_formatting_print_statements;

fn main() {
    println!("=========================================");
    println!("PRINT FORMATTING");
    c_formatting_print_statements::formatting_print();

    println!("=========================================");
    println!("PRIMITIVE VARIABLES");
    b_primitive_data_types::primitive_data_types();
    println!("Integers --------------------------------");
    b_primitive_data_types::integers();
    println!("Floats --------------------------------");
    b_primitive_data_types::floating_points();
    println!("Arithmetic --------------------------------");
    b_primitive_data_types::arithmetic_operations();
    println!("Bitwise --------------------------------");
    b_primitive_data_types::bitwise_operations();
    println!("Boolean --------------------------------");
    b_primitive_data_types::boolean_operations();
    println!("Comparison --------------------------------");
    b_primitive_data_types::comparison_operations();
    println!("Char --------------------------------");
    b_primitive_data_types::chars();

    println!("=========================================");
    println!("HELLO WORLD");
    a_hello_world::hello_world();
}
