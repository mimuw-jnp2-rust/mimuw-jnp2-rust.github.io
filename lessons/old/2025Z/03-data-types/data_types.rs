#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position(i32, i32); // This is a "tuple struct".

// Could Hero derive the Copy trait?
#[derive(Clone, Debug, Eq, PartialEq)]
struct Hero {
    name: String,
    level: u32,
    experience: u32,
    position: Position,
}

// We can add methods to structs using the 'impl' keyword.
impl Hero {
    // Static method (in Rust nomenclature: "associated function").
    // It can then be called as follows: `Hero::new(String::from("Ferris"))`.
    fn new(name: String) -> Hero {
        Hero {
            name,
            level: 1,
            experience: 0,
            position: Position(0, 0),
        }
    }
}

// We can have multiple `impl` blocks for one struct.
impl Hero {
    // Instance method. The first argument (self) is the calling instance,
    // just like `self` in Python and `this` in C++.
    fn distance(&self, pos: Position) -> u32 {
        // For convenience, we don't have to type the argument as `self: &Self`.
        // The i-th field of a tuple or a tuple struct can be accessed through 'tuple.i'.
        // Do not abuse this syntax, though; it's often cleaner to perform
        // pattern matching to decompose the tuple.
        (pos.0 - self.position.0).unsigned_abs() + (pos.1 - self.position.1).unsigned_abs()
    }

    // Mutable borrow of self allows to change instance fields.
    fn level_up(&mut self) {
        // Again, we don't have to type the argument as `self: &mut Self`.
        self.experience = 0;
        self.level += 1;
    }

    // 'self' is not borrowed here and will be moved into the method.
    fn die(self) {
        println!(
            "Here lies {}, a hero who reached level {}. RIP.",
            self.name, self.level
        );
        // The `self: Self` is now dropped.
    }
}

fn main() {
    // Calling associated functions requires scope (`::`) operator.
    let mut hero: Hero = Hero::new(String::from("Ferris"));
    hero.level_up(); // 'self' is always passed implicitly as the first argument.

    // Thanks to `..hero`, fields other than 'name' will be the same as in 'hero'.
    // In general, they are moved. Here, they are copied, because all missing fields
    // implement the `Copy` trait.
    let steve = Hero {
        name: String::from("Steve The Normal Guy"),
        ..hero
    };

    assert_eq!(hero.level, steve.level);

    let mut twin = hero.clone();

    // We can compare `Hero` objects because it derives the `PartialEq` trait.
    assert_eq!(hero, twin);
    twin.level_up();
    assert_ne!(hero, twin);
    hero.level_up();
    assert_eq!(hero, twin);

    // We can print out the struct's debug string
    // (which is implemented thanks to `Debug` trait) with '{:?}'.
    println!("print to stdout: {:?}", hero);

    hero.die(); // 'hero' is not usable after this invocation, see the method's definiton.

    // The `dbg!` macro prints debug strings to stderr along with file and line number.
    // `dbg!` takes its arguments by value, so it's better to borrow them to not have them
    // moved into `dbg!` and consumed.
    dbg!("print to stderr: {}", &twin);

    let pos = Position(42, 0);
    let dist = steve.distance(pos); // No clone here as `Position` derives the `Copy` trait.
    println!("{:?}", pos);
    assert_eq!(dist, 42);
}
