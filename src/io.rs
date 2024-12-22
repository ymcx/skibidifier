use std::{fs::File, io::{Read, Write, stdin}};

pub fn write_file(filename: &str, message: &str) {
    let mut file = File::create(filename).expect("Couldn't create the output file");
    file.write_all(message.as_bytes()).expect("Couldn't write to the output file");
}

pub fn write_std(message: &str) {
    println!("{}", message);
}

pub fn read_file(filename: &str) -> String {
    let mut buffer = String::new();
    let mut file = File::open(filename).expect("Couldn't open the input file");
    file.read_to_string(&mut buffer).expect("Couldn't read from the input file");
    buffer
}

pub fn read_std() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).expect("Couldn't read from STDIN");
    buffer
}
