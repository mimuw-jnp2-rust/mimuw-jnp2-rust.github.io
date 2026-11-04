#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    let x = 40; // inferred type
    let y: i32 = 100; // specified type

    {
        let x = 40 + 2; // shadowing
        println!("x is {}", x); // prints 42
    }

    // x = 0; // compilation error, variables are by default immutable
    let mut x = 40; // declare as mutable
    x = 0; // now we can reassign

    x += 1; // x = x + 1
}
