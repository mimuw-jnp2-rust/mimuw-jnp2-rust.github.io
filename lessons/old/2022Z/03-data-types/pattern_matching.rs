#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // Pattern matching is basically a switch on steroids.
    let number = rand::random::<i32>();
    match number % 7 {
        0 => println!("{number} is divisible by 7"),
        1 => println!("{number} is *almost* divisible by 7"),
        _ => println!("{number} is not divisible by 7"),
    }

    #[derive(Debug)]
    enum Color {
        Pink,
        Brown,
        Lime,
    }

    let color = Color::Lime;
    match color {
        Color::Pink => println!("My favorite color!"),
        _ => println!("Not my favorite color!"), // _ is a wildcard
                                                 // Rust will statically check that we covered all cases or included a default case.
    }

    // We can also use pattern matching to match on multiple values.
    match (color, number % 7) {
        (Color::Pink, 0) => println!("My favorite color and number!"),
        (Color::Pink, _) => println!("My favorite color!"),
        (_, 0) => println!("My favorite number!"),
        (_, _) => println!("Not my favorite color or number!"),
    }
    // (This is not special syntax, we're just pattern matching tuples.)

    // But we can also *destructure* the value
    struct Human {
        age: u8,
        favorite_color: Color,
    }

    let john = Human {
        age: 42,
        favorite_color: Color::Pink,
    };

    match &john {
        Human {
            age: 42,
            favorite_color: Color::Pink,
        } => println!("Okay, that's John!"),
        Human {
            favorite_color: Color::Pink,
            ..
        } => println!("Not John, but still his favorite color!"),
        _ => println!("Somebody else?"),
    }

    // Note two things:
    // 1. Color is *not* Eq, so we can't use == to compare it, but pattern matching is fine.
    // 2. We *borrowed* the value, so we can use it after the match.

    println!("John is {} years old and still kicking!", john.age);

    // To save some time, we can use `if let` to match against only one thing
    // We could also use `while let ... {}` in the same way
    if let Color::Pink = &john.favorite_color {
        println!("He's also a man of great taste");
    }

    // We can match ranges...
    match john.age {
        0..=12 => println!("John is a kid!"),
        13..=19 => println!("John is a teenager!"),
        20..=29 => println!("John is a young adult!"),
        30..=49 => println!("John is an adult!"),
        50..=69 => println!("John is mature!"),
        _ => println!("John is old!"),
    }

    // We can use match and capture the value at the same time.
    match john.age {
        age @ 0..=12 => println!("John is a kid, age {}", age),
        age @ 13..=19 => println!("John is a teenager, age {}", age),
        age @ 20..=29 => println!("John is a young adult, age {}", age),
        age @ 30..=49 => println!("John is an adult, age {}", age),
        age @ 50..=69 => println!("John is mature, age {}", age),
        age => println!("John is old, age {}", age),
    }

    // We can use guards to check for multiple conditions.
    match john.age {
        age @ 12..=19 if age % 2 == 1 => println!("John is an *odd* teenager, age {}", age),
        age if age % 2 == 0 => println!("John is an *even* man, age {}", age),
        _ => println!("John is normal"),
    }

    // Finally, let's look at some references now
    let reference: &i32 = &4;

    match reference {
        &val => println!("Value under reference is: {}", val),
    }

    // `ref` can be used to create a reference when destructuring
    let Human {
        age,
        ref favorite_color,
    } = john;
    // `john` is still valid, because we borrowed using `ref`
    if let Color::Pink = &john.favorite_color {
        println!("John still has his color - {:?}!", favorite_color);
    }

    let mut john = john;

    // `ref mut` borrows mutably
    let Human {
        age,
        ref mut favorite_color,
    } = john;
    // We use `*` to dereference
    *favorite_color = Color::Brown;
    println!(
        "Tastes do change with time and John likes {:?} now.",
        john.favorite_color
    );
}
