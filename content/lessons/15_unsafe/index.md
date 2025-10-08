+++
title = "Unsafe Rust"
date = 2029-01-01
weight = 1
[extra]
lesson_date = 2029-01-01
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

The single fundamental property of Safe Rust, _the soundness property_:

**No matter what, Safe Rust can't cause Undefined Behavior.**

This is a valid _sound_ code, with a safe encapsulation over `unsafe` interior.

```rust
fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx < arr.len() {
        unsafe {
            Some(*arr.get_unchecked(idx))
        }
    } else {
        None
    }
}
```

_(Un)soundness_ means that there exists a _possibility_ to trigger UB.
The following code is _unsound_ (why? what has changed?):

```rust
fn index(idx: usize, arr: &[u8]) -> Option<u8> {
    if idx <= arr.len() {
        unsafe {
            Some(*arr.get_unchecked(idx))
        }
    } else {
        None
    }
}
```

But we only changed safe code! This shows that `unsafe` is unfortunately not perfectly scoped and isolated. We need to be extra careful when writing `unsafe` code.

## Reading

- [The Book, Chapter 19.1](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/), especially chapter 1 _(Meet Safe and Unsafe)_

- [How unpleasant is Unsafe Rust?](https://www.reddit.com/r/rust/comments/16i8lo2/how_unpleasant_is_unsafe_rust/)

- [RUDRA: Finding Memory Safety Bugs in Rust at the Ecosystem Scale](https://taesoo.kim/pubs/2021/bae:rudra.pdf) - automatic static analyzer to find 3 most frequent subtle bugs in `unsafe` code:
  1. panic (unwind) safety bug (analogous to exception-handling guarantees in C++),
  2. higher-order safety invariant (assuming certain properties of the type that the generic is instantiated with that are not guaranteed by the type system, e.g., _purity_),
  3. propagating Send/Sync in Generic Types (implementing Send/Sync unconditionally for T, even if T contains non-Send/non-Sync types inside).

  **RUDRA found 264 previously unknown memory-safety bugs in 145 packages on crates.io!!!**

  Is Rust really a safe language...?

  Only transitively. _Safe Rust_ is sound iff `unsafe` code called by it is sound too.
