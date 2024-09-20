+++
title = "Closures and Iterators"
date = 2022-11-14
weight = 1
[extra]
lesson_date = 2024-10-31
+++

# Closures

Closures (Polish: "domknięcia") are anonymous functions that can access variables from the scope in which they were defined.

We'll go through the examples from [Rust by Example](https://doc.rust-lang.org/rust-by-example/fn/closures.html).

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
