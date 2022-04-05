+++
title = "Small Task Feedback #2"
date = 2022-04-04
weight = 1
[extra]
lesson_date = 2022-04-05
+++

## Result combinators

Rust's `Result` type implements a lot of methods that simplify working with the two variants of the enum, especially when we're only interested in one of them. These methods are called _combinators_.

{{ include_code_sample(path="lessons/9_feedback2/combinators.rs", language="rust") }}

You can find more about the `Result` type and all its methods [here](https://doc.rust-lang.org/std/result/enum.Result.html).

## Useful hashmap methods

We can create a new hashmap in two ways in Rust - either an empty one using the `new()` method or from a list of key-value pairs using the `from()` method.

```rust
let mut empty: HashMap<&str, i32> = HashMap::new();
let filled = HashMap::from([
    ("a", 1),
    ("b", 2),
    ("c", 3),
    ("d", 4),
]);
```

`std::collections::Hashmap` implements the `IntoIterator` trait, but there are also other very handy methods for iterating over the collection. We can use the `values()` and `values_mut()` methods to just iterate over values inside the map and the `keys()` method to iterate only over the keys.

```rust
let mut map = HashMap::from([("a", 1), ("b", 2)]);

map.values().for_each(|v| println!("{}", v));

for v in map.values_mut() {
    *v = 0;
}

map.keys().filter(|key| key.len() == 1).count();
```

We can also consume all the key-value pairs from the map using `drain()`.

```rust
use std::collections::HashMap;

let mut a = HashMap::new();
a.insert(1, "a");
a.insert(2, "b");

for (k, v) in a.drain().take(1) {
    assert!(k == 1 || k == 2);
    assert!(v == "a" || v == "b");
}

assert!(a.is_empty());
```

In the previous feedback you can also read about the `Entry` enum and how to work with it to access and modify values in a hashmap.

## `matches!()` macro

Rust has a very convenient macro for checking whether something matches a given pattern. You can read more about the `matches!()` macro [here](https://doc.rust-lang.org/std/macro.matches.html).

## Assignment #4 (graded)

[Here](https://classroom.github.com/a/jL6DS9YM) you can find the fourth graded assignment. Deadline for submissions is 12.04.2022.
