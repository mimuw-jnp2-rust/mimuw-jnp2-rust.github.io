fn box_simple() {
    let b = Box::new(5);
    println!("b = {}", b);

    let _x = 10 + *b;
}

// `Box` gives us the indirection required to define
// recursive types
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    box_simple();
}
