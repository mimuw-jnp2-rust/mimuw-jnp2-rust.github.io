trait Speak {
    fn speak(&self) -> &'static str;
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) -> &'static str {
        "Hau hau" // It's a Polish dog!
    }
}

struct Human;

impl Speak for Human {
    fn speak(&self) -> &'static str {
        "Hello world"
    }
}

// It works like templates in C++.
// A different function will be generated for each T during compilation.
// This process is called "monomorphization".
fn static_dispatch<T: Speak>(speaking: &T) {
    println!("{}!", speaking.speak());
}

// Only one copy of that function will exist in the compiled binary.
fn dynamic_dispatch(speaking: &dyn Speak) {
    println!("{}!", speaking.speak());
}

fn main() {
    let dog = Dog;
    let human = Human;

    static_dispatch(&dog);
    static_dispatch(&human);

    dynamic_dispatch(&dog);
    dynamic_dispatch(&human);

    // The observable behavior is identical.
    // Static dispatch in general is a bit faster,
    // because there is no need to perform a "vtable lookup".
    // But it can also result in bigger binary sizes.
}
