#![allow(dead_code)]

use std::fmt::Debug;

// Generic enums.
enum OurOption<T> {
    Some(T),
    None,
}

// Generic structs.
struct Tuple2<T, U> {
    x: T,
    y: U,
}

// Generic implementation.
impl<T, U> Tuple2<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// Conditional implementation.
impl<T: PartialOrd + Copy> Pair<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

// Alternative syntax.
impl<T> Pair<T>
where
    T: PartialOrd + Copy,
{
    fn smallest(&self) -> T {
        if self.x < self.y {
            self.x
        } else {
            self.y
        }
    }
}

// The information about the concrete underlying type is preserved.
// If I call it with a `String`, then I get back a `String`.
fn cloning_machine<T: Clone + Debug>(item: &T) -> T {
    item.clone()
}

// The information about the concrete underlying type is erased.
// We can only either format or clone the result.
// If I call it with a `String`, then I'll only know that the return type
// implements `Clone + Debug`.
fn erasing_cloning_machine2<T: Clone + Debug>(item: &T) -> impl Clone + Debug {
    item.clone()
}

// The returned type behaves exactly the same as above (it's the same type, after all)
// and the function has the same requirements for the `item` argument.
// But inside the implementation of the function, we can't use `T` (it's not defined anywhere).
fn erasing_cloning_machine1(item: &(impl Clone + Debug)) -> impl Clone + Debug {
    item.clone()
}

fn main() {
    let _opt = OurOption::Some(10);

    let _p1 = Tuple2 { x: 5, y: 10 };
    let _p2 = Tuple2::new(1, 2.5);

    let arr = [1, 2, 3];

    let arr2 = cloning_machine(&arr);
    let _x = arr2[0]; // This compiles, because `cloning_machine` preserves the type.
    println!("{:?}", arr2);

    let arr3 = erasing_cloning_machine1(&arr);
    // arr3[0]; // won't compile: cannot index into a value of type `impl std::clone::Clone + std::fmt::Debug`
    println!("{:?}", arr3);

    let arr4 = erasing_cloning_machine2(&arr);
    // arr4[0]; // won't compile: cannot index into a value of type `impl std::clone::Clone + std::fmt::Debug`
    println!("{:?}", arr4);
}
