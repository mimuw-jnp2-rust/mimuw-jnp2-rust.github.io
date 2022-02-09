+++
title = "Introduction to Rust"
date = 2022-02-16
weight = 1
[extra]
lesson_date = 2022-03-01
+++

![Logo](https://www.rust-lang.org/logos/rust-logo-blk.svg)

## Why use Rust?

- It is **safe** (compared to C++ for example, as we will see in a minute)
- It is **fast** (because it is compiled to machine code)
- It is ergonomic and pleasant to use (static typing, expressive type system, helpful compiler warnings)
- It is [loved by programmers](https://insights.stackoverflow.com/survey/2021#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)
- It provides excellent tooling

## Why learn Rust?
Even if you don't end up using Rust, learning it expands your horizons 
  * it helps especially with the awareness of what you can and can't do in concurrent applications
  * it helps you understand the memory management

## Why not learn Rust?
- Some people say Rust is too hard to learn because of the borrow checker
- Once you get to know Cargo you won't ever want to use a language without a built-in package manager ;)
- You will start hating C++ (Piotrek, don't punch me!)

## Demo
Let's compare the same code written in [C++](demo.cpp) and [Rust](demo.rs).

## Installing Rust
- [Rustup](https://rustup.rs/)
- Setup an IDE 
  * [CLion](https://www.jetbrains.com/clion/) (you can get it [for free](https://www.jetbrains.com/community/education/#students)) and [Intellij-Rust](https://intellij-rust.github.io/)
  * [VSCode](https://code.visualstudio.com/) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
  * rust-analyzer also works with [other IDEs](https://rust-analyzer.github.io/manual.html#installation)

## Useful tools
![Clippy](clippy.jpg)

- `cargo clippy` (for static analysis)
- there's also `cargo check`, but it's less powerful than clippy
- `cargo fmt` (for code formatting)

## Hello world
See [this](hello-world.rs) example.

## Test assignment (not graded)
Click [here](https://classroom.github.com/a/sFJOi1pT)

## Additional reading
- https://doc.rust-lang.org/stable/rust-by-example/