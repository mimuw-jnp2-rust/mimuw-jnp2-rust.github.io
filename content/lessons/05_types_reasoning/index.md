+++
title = "Reasoning About Types"
date = 2024-10-23
weight = 1
[extra]   
lesson_date = 2024-10-24
+++

# Type traits

Traits are a way to defined common behavior between different types. They can be compared to _interfaces_ from many other mainstream languages or to type classes from Haskell, however, Rust is not an object-oriented language and there are some notable differences between type traits and Java interfaces.

The way we describe behavior in Rust is through methods. Traits consist of a set of these methods which then should be implemented by a type. We've already encountered examples of these, like the `Clone` trait which specified that the `clone()` method can be called on some given type. Now, let's take a deeper look and try defining our own trait.

{{ include_code_sample(path="lessons/05_types_reasoning/basic_trait.rs", language="rust") }}

## Default implementations

Trait definitions can also be provided with default implementations of behaviors.

{{ include_code_sample(path="lessons/05_types_reasoning/trait_default.rs", language="rust") }}

## What about _derive_?

There is a trait-related thing we have used quite extensively and not explained yet, namely the `#[derive]` attribute. What it does is generate items (in our case a trait implementation) based on the given data definition (here a struct). Below you can find a list of derivable traits from the standard library. Writing derivation rules for user defined traits is also possible, but goes out of the scope of this lesson.

Derivable traits:

- Equality traits: `Eq`, `PartialEq` and comparison traits: `Ord` and `PartialOrd`. The `Partial-` versions exist because there are types which don't fulfill the reflexivity requirement of equality (`NaN != NaN`) or do not form a total order (` NaN < 0.0 == false` and `NaN >= 0.0 == false`).

- Data duplication traits: `Clone` and `Copy`

- `Hash` - allows using values of that type as keys in a hashmap

- `Default` - provides a zero-arg constructor function

- `Debug` - provides a formatting of the value which can be used in debugging context. It should _NOT_ be implemented manually. In general, if it's possible to derive the `Debug`, there are no reasons against doing it.

### When is it possible to derive a trait?

When all fields of a struct/variants of an enum implement that trait.

### Should all traits always be derived if it is possible?

No. Although it may be tempting to just slap `#[derive(Clone, Copy)]` everywhere, it would be counter-effective. For example, at some later point you might add a non-Copy field to the struct and your (or, what's worse, someone else's!) code would break. Another example: it makes little sense to use containers as keys in hashmaps or to compare tweets.

# Generics

Suppose we want to find the largest element in a sequence and return it. Very much on purpose, we didn't specify what type these elements would be - ideally, we would love it to work on all types that have a defined notion of a _largest_ element. However, to make things simpler for now, let's focus only on two primitive types: `i32` and `char`. Let's try to write the code:

{{ include_code_sample(path="lessons/05_types_reasoning/non_generic.rs", language="rust") }}

Perfect, it works! Now only twenty more types to go...

Fortunately, Rust gives us a way to avoid all this code duplication and generalize the types we're working on.

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

Cleaner already - we merged possibly very many implementations into one. But, when we try to compile this:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ++++++++++++++++++++++
```

Since `T` can be of absolutely any type now, the compiler cannot be sure that operator `>` is defined. This aligns with what we wanted, as without comparing elements we don't have a notion of the largest one either. As always, the compiler comes to our aid:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

We call this a _trait bound_, a way to provide constraints on what kind of types we are talking about in a given context. This implementation almost works now. Let's look at the new error.

```
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`
```

Our function attempts to take ownership, but, again, the compiler doesn't know whether `T` can just be trivially copied. Rust allows us to combine multiple trait bounds together:

{{ include_code_sample(path="lessons/05_types_reasoning/generic_largest.rs", language="rust") }}

## A powerful tool

There's a lot more that we can do with generics:

{{ include_code_sample(path="lessons/05_types_reasoning/generics.rs", language="rust") }}

A bit more involved example:

{{ include_code_sample(path="lessons/05_types_reasoning/generics_fun.rs", language="rust") }}

## Static vs dynamic dispatch

{{ include_code_sample(path="lessons/05_types_reasoning/static_dynamic_dispatch.rs", language="rust") }}

# Lifetimes

Going back to the lesson about ownership, if we try to compile the following code:

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

we should expect to get an error:

```
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  |
10 |         println!("r: {}", r);
   |                           - borrow later used here
```

Courtesy of the borrow checker, we didn't end up with a dangling reference. But what exactly is happening behind the scenes? Rust introduces a concept of annotated lifetimes, where the lifetime of each value is being marked and tracked by the checker. Let's look at some examples:

```rust
{
    let r;                  // ---------+-- 'a
                            //          |
    {                       //          |
        let x = 5;          // -+-- 'b  |
        r = &x;             //  |       |
    }                       // -+       |
                            //          |
    println!("r: {}", r);   //          |
}                           // ---------+
```

```rust
{
    let x = 5;              // ----------+-- 'b
                            //           |
    let r = &x;             // --+-- 'a  |
                            //   |       |
    println!("r: {}", r);   //   |       |
                            // --+       |
}                           // ----------+
```

## Annotations

Let's consider the following code finding the longer out of two strings:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

If we try to compile this, we will get an error:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

This is because Rust doesn't know which of the two provided strings (`x` or `y`) will be returned from the function. And because they potentially have different lifetimes, the lifetime of what we are returning remains unclear to the compiler - it needs our help.

Rust provides syntax for specifying lifetimes. The lifetime parameter name from the example (`a`) doesn't have any concrete meaning - it's just an arbitrary name for this one lifetime.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

So, knowing this, let's address the compiler's demands.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

When working with lifetimes, our work will usually revolve around specifying relationships between lifetimes of different values so that the compiler can successfully reason about the program's safety. In the context of the example above, this signature means that both of the function's arguments and its output will live at least as long as lifetime `'a`. In practice, this means that the output's lifetime will be equal to the smaller of the two inputs' lifetimes.

{{ include_code_sample(path="lessons/05_types_reasoning/lifetimes_basic.rs", language="rust") }}

Trying to compile the second variant displeases the compiler (just like we hoped).

```
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

## Lifetime elision

We now know how to explicitly write lifetime parameters, but you might recall that we don't always have to that. Indeed, Rust will first try to figure out the lifetimes itself, applying a set of predefined rules. We call this _lifetime elision_.

{{ include_code_sample(path="lessons/05_types_reasoning/lifetimes_elision.rs", language="rust") }}

The above works, even though we didn't specify any lifetime parameters at all. The reason lies in the rules we mentioned, which are as follows (where input lifetimes are lifetimes on parameters and output lifetimes are lifetimes on return values):

- Each parameter that is a reference gets its own lifetime parameter.

- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.

- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

Let's try to understand how the compiler inferred the lifetimes of our `first_two` functions. We start with the following signature:

```rust
fn first_two(seq: &[u32]) -> &[u32] {
```

Then, we apply the first rule:

```rust
fn first_two<'a>(seq: &'a [u32]) -> &[u32] {
```

Next, we check the second rule. It applies here as well.

```rust
fn first_two<'a>(seq: &'a [u32]) -> &'a [u32] {
```

With that, we arrive at a state where all lifetimes are specified.

## Static lifetime

There exists one special lifetime called `'static`, which means that a reference can live for the entire duration of the program. All string literals are annotated with this lifetime as they are stored directly in the program's binary. Full type annotation of a string literal in Rust is therefore as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```

# Obligatory reading

- [The Book, chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html)

- [Polymorphism in Rust](https://oswalt.dev/2021/06/polymorphism-in-rust/)
