+++
title = "Introduction to Rust"
date = 2024-09-20
weight = 1
[extra]
lesson_date = 2024-10-03
+++

![Logo](https://www.rust-lang.org/logos/rust-logo-blk.svg)

# A language empowering everyone to build reliable and efficient software.

([unofficial logo](https://rustacean.net/))

## Why use Rust?

- It is **safe** (compared to C++ for example, as we will see in a minute)
- It is **fast** (because it is compiled to machine code)
- It is ergonomic and pleasant to use (static typing, expressive type system, helpful compiler
  warnings)
- It
  is [loved by programmers](https://insights.stackoverflow.com/survey/2021#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)
- It provides excellent tooling

## Why learn Rust?

Even if you don't end up using Rust, learning it expands your horizons

- it helps especially with the awareness of what you can and can't do in concurrent applications
- it helps you understand memory management and learn its good practices

## Why not to learn Rust?

- Some people say Rust is too hard to learn because of the borrow checker
- Once you get to know Cargo you won't ever want to use a language without a built-in package
  manager ;)
- You will start hating C++

## Demos

![Meme](cpp_meme.jpg)

Let's compare the same code written in [C](errors_demo.c), [C++](errors_demo.cpp)
and [Rust](errors_demo.rs).

## Code you sent in previous editions!

### Aleksander Tudruj

{{ include_code_sample(path="lessons/01_introduction/students/tudruj.cpp", language="cpp") }}

### Krystyna Gasińska

{{ include_code_sample(path="lessons/01_introduction/students/gasinska.py", language="python") }}

### Antoni Koszowski

{{ include_code_sample(path="lessons/01_introduction/students/koszowski.go", language="go") }}

### Mieszko Grodzicki

{{ include_code_sample(path="lessons/01_introduction/students/grodzicki.py", language="python") }}

## Installing Rust

- [Rustup](https://rustup.rs/)
- Setup an IDE
  - [CLion](https://www.jetbrains.com/clion/) (you can get
    it [for free](https://www.jetbrains.com/community/education/))
    and [RustRover](https://intellij-rust.github.io/)
  - [VSCode](https://code.visualstudio.com/)
    and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
  - rust-analyzer also works
    with [other IDEs](https://rust-analyzer.github.io/manual.html#installation)

## Useful tools

![Clippy](clippy.jpg)

- `cargo clippy` (for static analysis)
- there's also `cargo check`, but it's less powerful than clippy
- `cargo fmt` (for code formatting)

### Rust Playground

- [online Rust compiler](https://play.rust-lang.org/)

## Hello world

{{ include_code_sample(path="lessons/old/2021L/01_introduction/hello_world.rs", language="rust") }}

### Variables

{{ include_code_sample(path="lessons/old/2021L/01_introduction/variables.rs", language="rust") }}

### Conditionals

{{ include_code_sample(path="lessons/old/2021L/01_introduction/conditionals.rs", language="rust") }}

### Loops

{{ include_code_sample(path="lessons/old/2021L/01_introduction/loops.rs", language="rust") }}

### Functions

{{ include_code_sample(path="lessons/old/2021L/01_introduction/functions.rs", language="rust") }}

## Test assignment (not graded)

Click [here](https://classroom.github.com/a/l3iF_TJU)

## Obligatory reading

- [The Book, chapters 1-3](https://doc.rust-lang.org/stable/book/)

## Additional reading

- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
