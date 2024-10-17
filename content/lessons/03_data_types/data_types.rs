#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position(i32, i32); // tuple struct

// Could Hero derive the Copy trait?
#[derive(Clone, Debug, Eq, PartialEq)]
struct Hero {
    name: String,
    level: u32,
    experience: u32,
    position: Position,
}

// we can add methods to structs using the 'impl' keyword
impl Hero {
    // static method (in Rust nomenclature: "associated function")
    fn new(name: String) -> Hero {
        Hero {
            name,
            level: 1,
            experience: 0,
            position: Position(0, 0),
        }
    }
}

// multiple impl blocks are possible for one struct
impl Hero {
    // instance method, first argument (self) is the calling instance
    fn distance(&self, pos: Position) -> u32 { // shorthand to: `self: &Self`
        // field `i` of a tuple or a tuple struct can be accessed through 'tuple.i'
        (pos.0 - self.position.0).unsigned_abs() + (pos.1 - self.position.1).unsigned_abs()
    }

    // mutable borrow of self allows to change instance fields
    fn level_up(&mut self) { // shorthand to: `self: &mut Self`
        self.experience = 0;
        self.level += 1;
    }

    // 'self' is not borrowed here and will be moved into the method
    fn die(self) { // shorthand to: `self: Self`
        println!(
            "Here lies {}, a hero who reached level {}. RIP.",
            self.name, self.level
        );
    }
}

fn main() {
    // Calling associated functions requires scope (`::`) operator.
    let mut hero: Hero = Hero::new(String::from("Ferris"));
    hero.level_up(); // 'self' is always passed implicitly

    // fields other than 'name' will be the same as in 'hero'
    let steve = Hero {
        name: String::from("Steve The Normal Guy"),
        ..hero
    };

    assert_eq!(hero.level, steve.level);

    let mut twin = hero.clone();

    // we can compare Hero objects because it derives the PartialEq trait
    assert_eq!(hero, twin);
    twin.level_up();
    assert_ne!(hero, twin);
    hero.level_up();
    assert_eq!(hero, twin);

    // we can print out a the struct's debug string with '{:?}'
    println!("print to stdout: {:?}", hero);

    hero.die(); // 'hero' is not usable after this invocation, see the method's definiton

    // the dbg! macro prints debug strings to stderr along with file and line number
    // dbg! takes its arguments by value, so better borrow them not to have them
    // moved into dbg! and consumed.
    dbg!("print to stderr: {}", &twin);

    let pos = Position(42, 0);
    let dist = steve.distance(pos); // no clone here as Position derives the Copy trait
    println!("{:?}", pos);
    assert_eq!(dist, 42);
}
