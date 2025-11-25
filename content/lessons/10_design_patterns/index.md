+++
title = "Design patterns"
date = 2025-11-25
weight = 1
[extra]
lesson_date = 2025-11-16
+++

## Object-oriented programming and Rust

The book has [a chapter dedicated to it](https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html).
Especially the ["typestate"](https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types) pattern is very interesting.
Read more about it [here](http://cliffle.com/blog/rust-typestate/).

## How to build a good library

[These guidelines](https://rust-lang.github.io/api-guidelines/about.html) have been created by the Rust library team.

## How to handle errors

[This post](https://nick.groenen.me/posts/rust-error-handling/) is from 2020, but the libraries it mentions (`anyhow` and `thiserror`) are still the most popular.

## Serde

[Serde](https://serde.rs/) is the most popular serialization library for Rust.
It contains a lot of serializable/deserializable data formats
See the [README.md of serde_json](https://github.com/serde-rs/json) to get a feel of the API for those formats.

## Clap

For a lot of apps, it's convenient to use the polished [clap](https://docs.rs/clap/latest/clap/) library to handle the CLI.

## No new assignments this week

Please work on the first iteration of the big project instead.

NOTE: Remember that last week's project has a two week deadline, so you should submit it before the **next** lesson.
