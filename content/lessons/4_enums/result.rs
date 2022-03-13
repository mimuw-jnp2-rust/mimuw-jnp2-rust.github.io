#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io;
use std::io::Read;

// Let's try reading from a file.
// Obviously this can fail.
fn first_try() -> io::Result<()> {
    let file = File::open("/dev/random");
    match file {
        Ok(mut file) => {
            // We got a file!
            let mut buffer = vec![0; 128];
            let bytes_read = file.read(&mut buffer);
            // Here we could match again, but it would quickly become tedious.
            // So let's use the `?` operator instead.
            Ok(())
        }
        Err(error) => {
            Err(error) // This is needed in order to change the type from `io::Result<File>` to `io::Result<()>`
        }
    }
}

// The '?' operator allows us to return early in case of an error
// (it automatically converts the error type)
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
