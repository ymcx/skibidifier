use std::{env::args, process::exit};
use io::{read_file, read_std, write_file, write_std};
use crypt::crypt;

mod io;
mod crypt;

#[derive(Default)]
struct Data {
    pub encrypting: bool,
    pub decrypting: bool,
    pub outputfile: String,
    pub input:      String,
    pub key:        String,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        help();
    }

    let mut data = Data::default();
    let mut skip = false;

    for i in 1..args.len() {
        if skip {
            skip = false;
            continue;
        }
        match args[i].as_str() {
            "-K"|"--keyfile" => {
                skip = true;
                data.key = read_file(&args[i+1]);
            },
            "-I"|"--inputfile" => {
                skip = true;
                data.input = read_file(&args[i+1]);
            },
            "-O"|"--outputfile" => {
                skip = true;
                data.outputfile = args[i+1].to_string();
            },
            "-k"|"--key" => {
                skip = true;
                data.key = args[i+1].to_string();
            },
            "-i"|"--input" => {
                skip = true;
                data.input = args[i+1].to_string();
            },
            "-"|"--stdin" => {
                data.input = read_std();
            },
            "-d"|"--decrypt" => {
                data.decrypting = true;
            },
            "-e"|"--encrypt" => {
                data.encrypting = true;
            },
            "-h"|"--help" => {
                help();
            },
            _ => {}
        }
    }

    // Get the encrypted/decrypted message derived using the given key
    let message = crypt(data.encrypting, &data.key, &data.input);

    // Write the message to STDOUT or the chosen file
    if data.outputfile.is_empty() {
        write_std(&message);
    } else {
        write_file(&data.outputfile, &message);
    }
}

fn help() {
    println!("\
-h,          --help,                Print the help menu, i.e. the one you're viewing right now
-K <FILE>,   --keyfile    <FILE>,   Set the file from which the derivation key will be read
-I <FILE>,   --inputfile  <FILE>,   Set the file from which the input message will be read
-O <FILE>,   --outputfile <FILE>,   Set the file where the output will be saved
-k <STRING>, --key        <STRING>, Set the derivation key
-i <STRING>, --input      <STRING>, Set the input message
-,           --stdin,               Read the input message from STDIN
-d,          --decrypt,             Decrypt rather than encrypt
-e,          --encrypt,             Encrypt rather than decrypt

The derivation key consists of four 32-bit unsigned integers, separated by spaces.");
    exit(0);
}
