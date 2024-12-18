+++
title = "Unsafe Rust"
date = 2024-12-18 
weight = 1
[extra]
lesson_date = 2024-12-19 
+++

# Unsafe Rust Alter Ego

So far, no operation in Rust that we performed could trigger UB (Undefined Behaviour):

- data races were prevented by _sharing XOR mutability_ borrow checker rule;
- use-after-free, dangling references, etc. were prevented by lifetimes & ownership.

But no respectable, powerful programming language can stand being constrained that much, in such a cage!

In Rust, `unsafe` keyworld unleashes the hidden superpowers.

## Unsafe code superpowers

Inside a `unsafe { ... }` block, you can (and normally you can't):

- **Dereference a raw pointer,**
- **Call an unsafe function or method,**
- Access or modify a mutable static variable,
- Implement an unsafe trait,
- Access fields of a union.

The first superpower is the most important. (Efficient) implementation of many data structures would be impossible without ability to use raw pointers, as references don't allow circular dependencies, among other limitations.

In the following code sample, we show all superpowers of `unsafe` code:

{{ include_code_sample(path="lessons/15_unsafe/unsafe_superpowers.rs", language="rust") }}

## Safe code guarantees

Safe code may **_never_** cause Undefined Behaviour.

## Reading

- [The Book, Chapter 19.1](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/), especially chapter 1 _(Meet Safe and Unsafe)_

- [How unpleasant is Unsafe Rust?](https://www.reddit.com/r/rust/comments/16i8lo2/how_unpleasant_is_unsafe_rust/)
