+++
title = "Data Types"
date = 2029-01-01
weight = 1
[extra]
lesson_date = 2029-01-01
+++

## Aggregating data

Below is a compact overview of Rust's structs.

{{ include_code_sample(path="lessons/03_data_types/data_types.rs", language="rust") }}

## Enums

It is often the case that we want to define a variable that can only take
a certain set of values and the values are known up front.
Just like the below code shows, in C you can use an `enum` for this.

{{ include_code_sample(path="lessons/03_data_types/enums.c", language="c") }}

However, in C enums are just integers. Nothing prevents us from writing

```c
int main() {
    enum shirt_size my_size = 666;
    print_size(my_size);
}
```

C++ introduces enum classes which are type-safe. Legacy enums are also somewhat safer than in C (same code as above):

```
<source>:27:31: error: invalid conversion from 'int' to 'shirt_size' [-fpermissive]
   27 |     enum shirt_size my_size = 666;
      |                               ^~~
      |                               |
      |                               int
```

Even though in this case we got an error, we can still write code that somehow casts the integer to the enum.

## Algebraic data types

Some programming languages (especially functional ones) allow programmers to define
enums which carry additional information. Such types are usually called `tagged unions`
or `algebraic data types`. This name can be new to you, but there's a chance that you
already used it (or something similar). It's pretty easy to understand it.

In C++ we can use `union` with an `enum` tag to define it:

{{ include_code_sample(path="lessons/03_data_types/tagged_union.cpp", language="cpp") }}

C++17 introduced a new feature called `variant` which generalizes this concept.
You can read more about it [here](https://en.cppreference.com/w/cpp/utility/variant).

Java has a more or less analogous feature called `sealed classes`
since [version 17](https://docs.oracle.com/en/java/javase/17/language/sealed-classes-and-interfaces.html.).

In Python, this is quite clean starting from Python 3.10.

{{ include_code_sample(path="lessons/03_data_types/variant.py", language="py") }}

## Enums in Rust

Let's see how they are defined in Rust.

{{ include_code_sample(path="lessons/03_data_types/enums.rs", language="rust") }}

In Rust, enums are a core feature of the language.
You may have heard that one of Rust's defining characteristics is
the absence of ["the billion dollar mistake"](https://en.wikipedia.org/wiki/Tony_Hoare#Apologies_and_retractions).
So what can we do to say that a value is missing if there is no `null`?

In Rust, we can use the `Option` type to represent the absence of a value.

Option is defined as:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `<T>` part is called the "type parameter" and it causes Option to be generic.
We won't go deeper into this for now.

The fact that variables which could be `null` in other languages have a different type in Rust is
the solution to the billion dollar mistake!

{{ include_code_sample(path="lessons/03_data_types/option.rs", language="rust") }}

## To discuss during class

- Why `enum` is considered a core feature of the language?
- How Rust `enum`s could save us during standard project refactors/library usages, which wouldn't be as safe in some other languages?
- People find Rust `enum`s convenient, to the degree where they miss this feature in other languages. What makes them so convenient?
- Let's see an example of a file with a big enum. Is the file readable? How is that balanced versus type safety?
  - [Implementation](https://github.com/rust-lang/rust/blob/master/compiler/rustc_const_eval/src/interpret/step.rs) of doing a single step in calculations of const values in the compiler.

## Pattern matching

Pattern matching is a powerful feature of Rust and many functional languages, but it's slowly making
its way into imperative languages like Java and Python as well.

{{ include_code_sample(path="lessons/03_data_types/pattern_matching.rs", language="rust") }}

## Result

We said there are no exceptions in Rust and panics mean errors which cannot be caught.
So how do we handle situations which can fail? That's where the `Result` type comes in.

{{ include_code_sample(path="lessons/03_data_types/result.rs", language="rust") }}

## To discuss during class

- So, why would the approach with `Result` be any cleaner than exceptions?
- Are there any other benefits of using this approach rather than exceptions?
- Look into the following libraries, are those interfaces convenient? How is that balanced versus type safety?
  - [Examples of CLI parsing](https://github.com/clap-rs/clap/tree/master/examples/tutorial_derive) from `clap` repository,
  - [README.md](https://github.com/serde-rs/json) with examples of the interface of JSON serialization/deserialization library,
  - [Documentation](https://doc.rust-lang.org/std/path/struct.Path.html) of filesystem path handling from the standard library,
  - [Implementation](https://github.com/BurntSushi/ripgrep/blob/master/crates/grep/examples/simplegrep.rs) of a third-party well-liked app [`ripgrep`](https://github.com/BurntSushi/ripgrep) written in Rust.

## Obligatory reading

- The Book, chapters [5](https://doc.rust-lang.org/book/ch05-00-structs.html),
  [6](https://doc.rust-lang.org/stable/book/ch06-00-enums.html),
  [8](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html)
  and [9](https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html)
- [Option docs](https://doc.rust-lang.org/std/option/)
- [Result docs](https://doc.rust-lang.org/std/result/)

## Assignment 2 (graded)

[Communications](https://classroom.github.com/a/gDraT0lo)

Deadline: 23.10.2024 23:59
