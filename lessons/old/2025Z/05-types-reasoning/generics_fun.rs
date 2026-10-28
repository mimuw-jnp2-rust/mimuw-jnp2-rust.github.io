use std::fmt::{Display, Formatter};

trait DefaultishablyPrintable<T> {
    fn defaultish_print()
    where
        T: Display + Default,
    {
        println!("{}", T::default())
    }
}

struct Foo;

struct Bar;

impl Display for Bar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("this is a bar")
    }
}

impl Default for Bar {
    fn default() -> Self {
        Bar // Well, we have no other choice.
    }
}

impl DefaultishablyPrintable<i32> for Foo {}

impl DefaultishablyPrintable<Bar> for Foo {}

fn main() {
    // By typing `Foo as DefaultishablyPrintable<i32>`,
    // we tell the compiler to treat this `Foo` struct as
    // only a `DefaultishablyPrintable<i32>` trait.
    <Foo as DefaultishablyPrintable<i32>>::defaultish_print();
    <Foo as DefaultishablyPrintable<Bar>>::defaultish_print();
}
