mod a_hello_world;
mod b_primitive_data_types;
mod c_formatting_print_statements;
mod d_compound_data_types;
mod e_functions;
mod f_program_flow_control;

fn main() {
    println!("=========================================");
    println!("PROGRAM FLOW CONTROL");
    f_program_flow_control::conditionals();
    f_program_flow_control::conditional_assignment();
    f_program_flow_control::loops();
    f_program_flow_control::while_loops();
    f_program_flow_control::for_loops();
    f_program_flow_control::nested_loops();

    println!("=========================================");
    println!("FUNCTIONS");
    e_functions::main();

    println!("=========================================");
    println!("COMPOUND DATA TYPES");
    d_compound_data_types::arrays();
    d_compound_data_types::multi_dimensional_arrays();
    d_compound_data_types::tuples();

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
