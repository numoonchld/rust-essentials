mod a_hello_world;
mod b_primitive_data_types;
mod c_formatting_print_statements;
mod d_compound_data_types;
mod e_functions;
mod f_program_flow_control;
mod g_ownership;
mod h_references;

fn main() {
    println!("=========================================");
    println!("REFERENCING");
    h_references::no_borrowing();
    h_references::borrowing_for_access();
    h_references::borrowing_for_modification();

    println!("=========================================");
    println!("OWNERSHIP");
    g_ownership::scope();
    println!("Shadow and scope --------------------------------");
    g_ownership::shadow_and_scope();
    println!("String data type --------------------------------");
    g_ownership::string_type();
    println!("Move and Clone --------------------------------");
    g_ownership::ownership_move();
    g_ownership::ownership_clone();
    g_ownership::ownership_copy();
    g_ownership::ownership_transfer_in_functions_stack_resource();
    g_ownership::ownership_transfer_in_functions_heap_resource();
    g_ownership::ownership_transfer_in_functions_heap_resource_clone();
    g_ownership::ownership_transfer_in_functions_heap_resource_return_ownership();

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
