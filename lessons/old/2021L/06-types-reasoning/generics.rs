#![allow(dead_code)]

use std::fmt::Debug;

// generic enums
enum OurOption<T> {
    Some(T),
    None,
}

// generic structs
struct Tuple2<T, U> {
    x: T,
    y: U,
}

// generic implementation
impl<T, U> Tuple2<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// conditional implementation
impl<T: PartialOrd + Copy> Pair<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

// alternative syntax
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

// Here information about the concrete underlying type is erased
// We can only either format or clone the result
fn cloning_machine(item: &(impl Clone + Debug)) -> impl Clone + Debug {
    item.clone()
}

fn main() {
    let _opt = OurOption::Some(10);

    let _p1 = Tuple2 { x: 5, y: 10 };
    let _p2 = Tuple2::new(1, 2.5);

    let arr = [1, 2, 3];
    let arr2 = cloning_machine(&arr);
    // arr2[0]; // won't compile: cannot index into a value of type `impl std::clone::Clone + std::fmt::Debug`
    println!("{:?}", arr2)
}
