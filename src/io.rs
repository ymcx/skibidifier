use std::{fs::File, io::{Read, Write, stdin}};

pub fn write_file(filename: &str, message: &str) {
    let mut file = File::create(filename).unwrap();
    file.write_all(message.as_bytes()).unwrap();
}

pub fn write_std(message: &str) {
    println!("{}", message);
}

pub fn read_file(filename: &str) -> String {
    let mut buffer = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    buffer
}

pub fn read_std() -> String {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    buffer
}
