// generic enums
enum OurOption<T> {
    Some(T),
    None,
}

// generic structs
struct Point<T, U> {
    x: T,
    y: U,
}

// generic implementation
impl<T, U> Point<T, U> {
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
fn smallest<T>(pair: &Pair<T>) -> T
    where T: PartialOrd + Copy
{
    if pair.x < pair.y {
        pair.x
    } else {
        pair.y
    }
}

// syntactic sugar for trait bounds
fn cloning_machine(item: &impl Clone) -> impl Clone {
    item.clone()
}

fn main() {
    let opt = OurOption::Some(10);

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point::new(1, 2.5);

    let arr = [1, 2, 3];
    let arr2 = cloning_machine(&arr);
}