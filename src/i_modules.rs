use std::io;

pub fn io_module() {
    let mut buffer = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);
}
