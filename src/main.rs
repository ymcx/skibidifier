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

#[derive(PartialEq)]
enum State {
    DEFAULT,
    KEYFILE,
    INPUTFILE,
    OUTPUTFILE,
    KEY,
    INPUT,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        help();
    }

    let mut data = Data::default();
    let mut state = State::DEFAULT;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-K"|"--keyfile"    => state = State::KEYFILE,
            "-I"|"--inputfile"  => state = State::INPUTFILE,
            "-O"|"--outputfile" => state = State::OUTPUTFILE,
            "-k"|"--key"        => state = State::KEY,
            "-i"|"--input"      => state = State::INPUT,
            "-" |"--stdin"      => data.input = read_std(),
            "-d"|"--decrypt"    => data.decrypting = true,
            "-e"|"--encrypt"    => data.encrypting = true,
            "-h"|"--help"       => help(),
            arg => {
                match state {
                    State::DEFAULT => {
                        panic!("Unrecognized arguments provided");
                    },
                    State::KEYFILE => {
                        state = State::DEFAULT;
                        if !data.key.is_empty() {
                            panic!("Can't set the key more than once");
                        }
                        data.key = read_file(&arg);
                    },
                    State::INPUTFILE => {
                        state = State::DEFAULT;
                        if !data.input.is_empty() {
                            panic!("Can't set the string to be encrypted/decrypted more than once");
                        }
                        data.input = read_file(&arg);
                    },
                    State::OUTPUTFILE => {
                        state = State::DEFAULT;
                        if !data.outputfile.is_empty() {
                            panic!("Can't set the output file more than once");
                        }
                        data.outputfile = arg.to_string();
                    },
                    State::KEY => {
                        state = State::DEFAULT;
                        if !data.key.is_empty() {
                            panic!("Can't set the key more than once");
                        }
                        data.key = arg.to_string();
                    },
                    State::INPUT => {
                        state = State::DEFAULT;
                        if !data.input.is_empty() {
                            panic!("Can't set the string to be encrypted/decrypted more than once");
                        }
                        data.input = arg.to_string();
                    },
                }
            }
        }
    }

    if state != State::DEFAULT {
        panic!("An argument wasn't provided for the last flag");
    }

    if data.encrypting && data.decrypting {
        panic!("Can't encrypt and decrypt at the same time");
    }
    
    if !data.encrypting && !data.decrypting {
        panic!("You'll need to specify if you want to encrypt or decrypt");
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
