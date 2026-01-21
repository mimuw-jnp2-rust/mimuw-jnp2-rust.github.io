#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]

/* unsafe superpower 1: dereferencing pointers. */
fn superpower_1() {
    let x = 42;

    // Implicit &T -> *const T conversion.
    let raw_ptr: *const i32 = &x;

    // An old way to directly create a pointer.
    let raw_ptr: *const i32 = std::ptr::addr_of!(x);

    // The new way to directly create a pointer.
    let raw_ptr: *const i32 = &raw const x;

    // Dereferencing a raw pointer requires an `unsafe` block.
    println!("Value: {}", unsafe { *raw_ptr });
}

/* unsafe superpower 2: calling an unsafe function. */
unsafe fn unsafe_function() {
    println!("This is an unsafe function!");
}

fn superpower_2() {
    unsafe {
        // Calling an unsafe function.
        unsafe_function();
    }
}

/* unsafe superpower 3: Accessing or modifying mutable static variable.
 * It is unsafe because it can lead to data races if accessed concurrently.
 * */

static mut COUNTER: i32 = 0;

fn increment_counter() {
    unsafe {
        // Accessing and modifying a mutable static variable
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }
}

fn superpower_3() {
    // This would cause UB: a data race.
    // std::thread::spawn(increment_counter);
    increment_counter();
}

/* unsafe superpower 4: Implementing unsafe traits.
 * It is unsafe because safe code is permitted to cause UB if an unsafe trait
 * is implemented for a type that should not implement it (think Send/Sync).
 * */

unsafe trait CanBeAtomic {
    fn safe_method_of_unsafe_trait(&self);
}

struct MyStruct {
    i: i32,
}

unsafe impl UnsafeTrait for MyStruct {
    fn safe_method_of_unsafe_trait(&self) {
        println!("Method called!");
    }
}

fn superpower_4() {
    let my_struct = MyStruct { i: 42 };

    // Calling a safe method from an unsafe trait
    my_struct.safe_method_of_unsafe_trait();
}

/* unsafe superpower 5: Accessing fields of a union.
 * It is unsafe because union can contain a different variant that we try to read,
 * so we could read some rubbish value.
 * */

union MyUnion {
    int_value: i32,
    bool_value: bool,
}

fn main() {
    let u = MyUnion { int_value: 42 };

    unsafe {
        // Accessing a field of a union
        println!("Union value as int: {}", u.int_value);

        // Would result in UB, as the compiler may assume that bool is either 0 or 1 underneath.
        // println!("Union value as bool: {}", u.bool_value);
    }
}
