#![allow(unused_variables)]

fn main() {
    for i in 0..10 {
        println!("i is {}", i); // i in [0, 10)
    }

    let mut x = 0;

    while x < 50 {
        x += 1;
    }

    let mut y = 0;
    let mut iterations = 0;
    loop {
        iterations += 1;
        if iterations % 2 == 0 {
            continue;
        }
        y += 1;
        if y == 10 {
            break;
        }
    }

    // we can use labels to refer to a specific loop
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // ends the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    // We can use break with a value.
    // Because loops are expressions too,
    // the value we break with will be returned from the functions
    let mut counter = 0;
    let value = loop {
        counter += 1;
        if counter == 10 {
            break 32;
        }
    };
}
