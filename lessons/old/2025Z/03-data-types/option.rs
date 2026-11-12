#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let mut not_null: i32 = 42;
    not_null = 43;
    // not_null = None; // This won't compile because it's a different type!

    let mut nullable: Option<i32> = Some(42);
    nullable = None;
    nullable = Some(43);

    // Such construction is rare, but possible.
    let mut double_nullable: Option<Option<i32>> = Some(Some(42));
    // This won't even compile because it's a different type.
    // assert_ne!(double_nullable, Some(42));
    double_nullable = None;
    double_nullable = Some(None);

    // `None` and `Some(None)` are different.
    // Why? How are enums represented in memory?
    assert_ne!(double_nullable, None);

    // Now recall that division by 0 *panics*.
    // A panic is an unrecoverable error.
    // It is not an exception!
    // And in Rust there are no exceptions, so there are no try/catch blocks.
    // Now let's imagine that we want to divide one number by another:
    fn divide(dividend: i32, divisor: i32) -> i32 {
        dividend / divisor
    }

    // We get the divisor from the user, so it can be 0.
    // We want to handle this situation gracefully,
    // as we don't want to crash the program.
    // We can do this by using the `Option<T>` type.
    fn safe_divide(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    // Fortunately, such a function is already included in the standard library.
    // We need to specify the type of `number` explicitly,
    // because `checked_div` is implemented for all integer types
    // and Rust won't know which type we want to use.
    let number: i32 = 42;
    assert_eq!(number.checked_div(2), Some(21));
    assert_eq!(number.checked_div(0), None);

    // Now let's imagine we search for a value in an array.
    let numbers = [1, 2, 3, 4, 5];
    let three = numbers.iter().copied().find(|&x| x == 3);
    assert_eq!(three, Some(3));
    let seven = numbers.iter().copied().find(|&x| x == 7);
    assert_eq!(seven, None);
    // We won't delve deeper into the details of how iterators work for now,
    // but the key takeaway is that there are no sentinel
    // or special values like `nullptr`/`std::iterator` in Rust.

    // Usually there are two kinds of methods:
    // - ones that will panic if the argument is incorrect,
    //   like: `numbers[8];` (this will panic),
    // - and `checked` ones that return an `Option`.
    assert_eq!(numbers.get(8), None);

    // We can use `unwrap` to get the value out of an `Option`,
    // but we must be absolutely sure that the `Option` is `Some`,
    // otherwise we'll get a panic,
    // like: `numbers.get(8).unwrap();` (this will panic).
    // Or we can provide a default value:
    assert_eq!(numbers.get(8).copied().unwrap_or(0), 0);

    // Usually instead of unwrapping we use pattern matching,
    // we'll get to this in a minute,
    // but first let's see what else we can do with an option.
    let number: Option<i32> = Some(42);
    // We can use `map` to transform the value inside an `Option`.
    let doubled = number.map(|x| x * 2);
    assert_eq!(doubled, Some(84));
    // We can use `flatten` to reduce one level of nesting.
    let nested = Some(Some(42));
    assert_eq!(nested.flatten(), Some(42));
    // We can use `and_then` to chain multiple options.
    // This operation is called `flatmap` in some languages.
    let chained = number
        .and_then(|x| x.checked_div(0))
        .and_then(|x| x.checked_div(2));
    assert_eq!(chained, None);

    // The last two things we'll cover here are `take` and `replace`.
    // They are important when dealing with non-`Copy` types.
    // The `take` will return the value inside an `Option`
    // and leave a `None` in its place.
    let mut option: Option<i32> = None;
    // Again, we need to specify the type.
    // Even though we want to say that there is no value inside the `Option`,
    // this absent value must have a concrete type!
    assert_eq!(option.take(), None);
    assert_eq!(option, None);

    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    // Also `replace` can be used to swap the value inside an `Option`.
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);
}
