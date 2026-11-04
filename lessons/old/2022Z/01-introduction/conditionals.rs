#![allow(unused_variables)]

fn main() {
    let x = 42;

    if x == 42 {
        println!("x is 42");
    } else if x == 43 {
        println!("x is 43");
    } else {
        println!("x is not 42 or 43");
    }

    // we can also use ifs as expressions
    let a_or_b = if x == 0 {
        "a" // notice no semicolon at the end
    } else {
        "b"
    };
}
