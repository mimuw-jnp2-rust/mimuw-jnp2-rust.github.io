#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io;
use std::io::Read;

// Let's try reading from a file.
// Obviously this can fail.
fn first_try() -> io::Result<String> {
    let file = File::open("/dev/random");
    match file {
        Ok(mut file) => {
            // We got a file!
            let mut buffer = vec![0; 128];
            // Matching each result quickly become tedious...
            // Later in this file there's syntactic sugar to make it cleaner.
            match file.read_exact(&mut buffer) {
                Ok(_) => {
                    let gibberish = String::from_utf8_lossy(&buffer);
                    Ok(gibberish.to_string())
                }
                Err(error) => Err(error),
            }
        }
        Err(error) => {
            // This is needed in order to change the type from
            // `io::Result<File>` to `io::Result<()>`.
            Err(error)
        }
    }
}

// The '?' operator allows us to return early in case of an error
// (it automatically converts the error type).
fn second_try(filename: &'static str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut buffer = vec![0; 128];
    file.read_exact(&mut buffer)?;
    let gibberish = String::from_utf8_lossy(&buffer);
    Ok(gibberish.to_string())
}

fn main() {
    let filenames = [
        "/dev/random",
        "/dev/null",
        "/dev/cpu",
        "/dev/fuse",
        "there_certainly_is_no_such_file",
    ];
    for filename in filenames {
        println!("Trying to read from '{}'", filename);
        match second_try(filename) {
            Ok(gibberish) => println!("{}", gibberish),
            Err(error) => println!("Error: {}", error),
        }
    }
}
