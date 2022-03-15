#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum NamedSize {
    Small,
    Medium,
    Large,
    XL,
}

#[derive(Debug)]
enum ShirtSize {
    Named(NamedSize),
    Numeric(u32),
}

fn main() {
    println!(
        "Isn't it strange that some clothes' sizes are adjectives like {:?},",
        ShirtSize::Named(NamedSize::Small)
    );
    println!(
        "but sometimes they are numbers like {:?}?",
        ShirtSize::Numeric(42)
    );
}
