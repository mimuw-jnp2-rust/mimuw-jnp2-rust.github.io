+++
title = "Table of content"
date = 2025-10-05
weight = 1
[extra]
lesson_date = 2025-10-05
+++

# Lessons have the following content & introduced concepts:

1. Introduction:
   - Rust syntax, common programming concepts (variables, conditionals, loops, functions, etc.);
   - A Taste Of Rust, slides introducing briefly to Rust's most distinctive features.

2. Ownership:
   - panicking - because it's everywhere (think `unwrap()`)
   - panicking vs Result - when to use which and how
   - Rust Ownership model compared to GC and C++'s
   - RAII
   - move semantics, clones, Copy
   - borrowing, aliasing XOR mutability
   - slicing, string literals

3. Data types & project structure:
   - structs
   - enums
   - pattern matching
   - modules, crates, packages, visibility, exports & imports

4. Traits
   - traits
   - `derive`
   - generics
   - trait bounds
   - static vs dynamic dispatch
   - lifetime annotations
   - lifetime elision

5. Functional features
   - closures
   - iterators
   - functional API on Option & Result

6. Heap management
   - Drop
   - Deref & deref coercion
   - Box
   - Rc
   - Interior mutability: Cell, RefCell
   - bonus: Cow

7. Fearless concurrency
   - concurrency vs parallelism - recap
   - thread spawning
   - `move` closures
   - panic no propagation
   - channels
   - mutexes, rwlocks
   - Send & Sync
   - atomics
   - data parallelism: Rayon

8. Design patterns
   - OOP in Rust, typestate pattern
   - API guidelines
   - error handling
   - serde

9. Async
   - Tokio tutorial

10. Async 2
    - reinventing Future
    - Pin'ning

11. Macros
    - declarative macros (macro_rules!)
    - procedural macros

12. Unsafe Rust
    - how to bypass borrow checker
    - why and when to use unsafe
    - safe/unsafe Rust guarantees
