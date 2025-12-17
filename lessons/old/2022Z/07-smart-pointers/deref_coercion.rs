use std::ops::Deref;

struct MyBox<T>(T);

// We won't be allocating anything on the heap here as it is not important here.
// We're only focusing on the dereference mechanisms.
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let int_box = MyBox::new(x);

    assert_eq!(5, *int_box);

    // String also implements the `Deref` trait.
    // In fact, String actually is a smart pointer.
    let s = String::from("I'm a smart pointer too");
    hello(&s);

    // Deref coercion can deal with multiple levels of indirection.
    let str_box = MyBox::new(String::from("Rust"));
    hello(&str_box);
}
