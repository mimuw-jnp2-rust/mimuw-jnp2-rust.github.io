mod one {
    pub struct Point {
        x: i32,
    }

    impl Point {
        pub fn new(x: i32) -> Point {
            Point { x }
        }

        pub fn x(&self) -> i32 {
            self.x
        }
    }

    impl Default for Point {
        fn default() -> Point {
            Point { x: 10 }
        }
    }
}

fn main() {
    // won't compile, can't initialize private fields
    // let p = one::Point {
    //     x: 1,
    // };
    let p = one::Point::new(1);
    // won't compile, x is private
    // println!("{}", p.x);
    println!("{}", p.x());
    let p = one::Point::default();
    println!("{}", p.x());
}
