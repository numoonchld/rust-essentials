use std::env;
use std::fs;
use std::io::prelude::*;

pub fn cli_arguments() {
    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument)
    }
}

pub fn file_read() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("Content is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }
}

pub fn file_write() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the moon in this decade\n");
    speech.push_str("and do the other things.\n");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();

    file.write(b"\npluto");
}
