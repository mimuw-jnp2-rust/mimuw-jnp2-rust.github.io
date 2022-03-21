+++
title = "Reasoning About Types"
date = 2022-03-20
weight = 1
[extra]
lesson_date = 2022-03-22
+++

# Type traits

Traits are a way to defined common behavior between different types. They can be compared to _interfaces_ from many other mainstream languages or to typeclasses from Haskell, however, Rust is not an object-oriented language and there are some notable differences between type traits and typeclasses.

The way we describe behavior in Rust is through methods. Traits comprise of a set of these methods which then can be implemented by a type. We've already encountered examples of these, like the `Clone` trait which specified that the `clone()` method can be called on some given type. Now, let's take a deeper look and try defining our own trait.

{{ include_code_sample(path="lessons/6_types_reasoning/basic_trait.rs", language="rust") }}

## Defaults

Trait definitions can also be provided with default implementions of behaviors.

{{ include_code_sample(path="lessons/6_types_reasoning/trait_default.rs", language="rust") }}

## What about _derive_?

All is good and dandy, but there is a trait related thing we have used quite extensively and not explained yet, namely the `#[derive]` attribute. What it does is generate items (in our case a trait implementation) based on the given data definition (here a struct). Below you can find a list of derivable traits from the standard library. Writing derivation rules for user defined traits is also possible, but goes out of the scope of this lesson.

Derivable traits:

- Comparison traits: `Eq`, `PartialEq`, `Ord` and `PartialOrd`

- Data duplication traits: `Clone` and `Copy`

- `Hash`

- `Default`

- `Debug`

# Generics

Suppose we want to find the largest element in a sequence and return it. Very much on purpose, we didn't specify what type these elements would be - ideally, we would love it to work on all types that have a defined notion of a _largest_ element. However, to make things simpler for now, let's focus only on two primitive types: `i32` and `char`. Let's try to write the code:

{{ include_code_sample(path="lessons/6_types_reasoning/non_generic.rs", language="rust") }}

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

{{ include_code_sample(path="lessons/6_types_reasoning/generic_largest.rs", language="rust") }}

## A powerful tool

There's a lot more that we can do with generics:

{{ include_code_sample(path="lessons/6_types_reasoning/generics.rs", language="rust") }}

# Lifetimes