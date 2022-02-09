fn main() {
    let name = "World";
    println!("Hello, {}!", name);

    // let another_name = "World" + 2; // ERROR

    let x = 40;
    {
        // Variable shadowing
        let x = 40 + 2;
        println!("x is {}", x);
    }
    // Immutable variables cannot be changed
    // x = 42; // ERROR
    println!("x is {}", x);

    #[allow(unused_assignments)]
    let mut x = 40;
    x = 42;
    println!("x is {}", x);

    x += 1;
    // x--; // ERROR

    if x == 42 {
        println!("x is 42");
    } else if x == 43 {
        println!("x is 43");
    } else {
        println!("x is not 42 or 43");
    }

    for i in 0..10 {
        println!("i is {}", i);
    }

    while x < 50 {
        x += 1;
    }

    println!("x is {}", x);

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

    dbg!(x, y);
}
