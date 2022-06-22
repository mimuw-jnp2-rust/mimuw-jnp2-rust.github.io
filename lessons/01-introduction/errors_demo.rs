fn main() {
    //# Syntax error
    // println("hello world");

    // let array = ["hello", "new", "world!"];

    //# Array out-of-bounds with a statically known index
    // println!("{}", array[3]);

    //# Array out-of-bounds with a dynamically computed index
    // for i in 0..=array.len() {
    //     println!("{}", array[i]);
    // }

    //# An unsuccessful attempt at emulating C++'s ability to read the memory we're not supposed to access
    // let format = "a very innocent hello {}";
    // println!(format);

    //# Division by zero
    // let joy_division = 0/0;

    // let joy = 0;
    // let joy_division = 0/joy;

    // let joy = if false {1} else {0};
    // let joy_division = 0/joy;

    // println!("{}", joy_division);
}
