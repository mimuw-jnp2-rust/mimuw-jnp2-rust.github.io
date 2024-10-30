+++
title = "Closures and Iterators"
date = 2024-10-30
weight = 1
[extra]
lesson_date = 2024-10-31
+++

# Closures

Closures (Polish: "domknięcia") are anonymous functions that can access variables from the scope in which they were defined.

## Closure syntax

{{ include_code_sample(path="lessons/06_closures_iterators/closures_syntax.rs", language="rust") }}

## Closures' types

Closures are unnameable types. That is, each closure gets its own unique type from the compiler,
but we cannot use it. Therefore, closures' types must be inferred.
We will often use `impl` keyword with closure traits (e.g., `impl Fn`) - those traits are described below.

## Closures capture environment

Closures can capture variables from the environment where they are defined. They can do that in two ways:
- Capturing References (borrowing), or
- Moving Ownership.

**HOW** closures capture variables is one thing.
But even more important is **WHAT** closures do with their captures.

{{ include_code_sample(path="lessons/06_closures_iterators/closures_capturing.rs", language="rust") }}

### Functions & closures hierarchy

Based on **WHAT** a closure does with its captures, it implements closure traits:

- `FnOnce` - closures that may move out of their captures environment (and thus called once).
- `FnMut` - closures that may mutate their captures, but don't move out of their captures environment (so can be called multiple times, but require a mutable reference);
- `Fn` - closures that do not mutate their captures (so can be called multiple times through an immutable reference).

For completeness, there is a (concrete) type of function pointers:
- `fn` - functions, closures with no captures.

Those traits and the `fn` type form a hierarchy: `fn` < `Fn` < `FnMut` < `FnOnce`

$$ fn \subseteq Fn \subseteq FnMut \subseteq FnOnce $$ 

## Examples

We'll go through the examples from [Rust by Example](https://doc.rust-lang.org/rust-by-example/fn/closures.html).
More examples will be seen when working with iterators.


# Iterators

In Rust, there is no hierarchy of types for collections (because there is no inheritance in general).
Instead, what makes a collection is that it can be iterated over.

We'll go through the official [docs](https://doc.rust-lang.org/stable/std/iter/).
Most methods are defined in the [Iterator trait](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html).

# Reading

- [The Book, chapter 12 (that's a project!)](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [The Book, chapter 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [The Book, chapter 14](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
- [The Book, Advanced Functions and Closures](https://doc.rust-lang.org/stable/book/ch19-05-advanced-functions-and-closures.html)
- [The Book, Advanced Traits](https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html)
