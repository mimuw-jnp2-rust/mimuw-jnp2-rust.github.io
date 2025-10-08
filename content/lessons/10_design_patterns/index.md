+++
title = "Design patterns"
date = 2029-01-01
weight = 1
[extra]
lesson_date = 2029-01-01
+++

## Object-oriented programming and Rust

The book has [a chapter dedicated to it](https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html).
Especially the ["typestate"](https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types) pattern is very interesting.
You can read more about it [here](http://cliffle.com/blog/rust-typestate/).

## How to build a good library

[These guidelines](https://rust-lang.github.io/api-guidelines/about.html) have been created by the Rust library team.

## How to handle errors

[This post](https://nick.groenen.me/posts/rust-error-handling/) is from 2020, but the libraries it mentions (`anyhow` and `thiserror`) are still the most popular.

## Serde

[Serde](https://serde.rs/) is the most popular serialization library for Rust.

## Assignment

This week's assignment is to write a "distributed" calculator.
You should base your solution on the [final project from the book](https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html).
