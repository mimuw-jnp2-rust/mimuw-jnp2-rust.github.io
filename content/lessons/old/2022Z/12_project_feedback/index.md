+++
title = "Project feedback"
date = 2022-12-29
weight = 1
[extra]
lesson_date = 2022-12-29
+++

# Project feedback

## Unwrapping options/results

Always ask yourself twice if you really need to unwrap. In most cases, you don't have to. Use pattern matching instead,
as it provides a static guarantee that the value is present.

Pattern matching prevents you from writing code like this:

```rust
fn main() {
    let x: Option<i32> = some_function();

    if x.is_some() {
        println!("x is {}", x.unwrap());
    }

    // Let's say this line was added later and/or you forgot to put it in the if statement.
    do_something(x.unwrap()); // this will blow up if x == None!
}
```

Instead, you can write:

```rust
fn main() {
    let x: Option<i32> = some_function();

    if let Some(x) = x {
        println!("x is {}", x);
        do_something(x);
    }
}
```

## Question mark operator

In methods that return `Result` or `Option`, you can use the question mark operator to return early if the value is `None` or `Err`.
See: [https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)

## Logging

You can use the [log](https://crates.io/crates/log) crate to log messages. It's better than `println!` because it
can be easily turned off. It also allows you to use different severity levels (e.g. `info`, `warn`, `error`) and only
log messages above a certain level.

## &String vs &str

See [https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)
In general, if you want to pass a reference to a string, use `&str` instead of `&String`.

## Use current versions of dependencies

You can use [cargo upgrades](https://crates.io/crates/cargo-upgrades) to check for outdated dependencies.

## If your project has separate binaries, use multiple binaries or a workspace

You can have multiple binaries in a single cargo project. Simply place them in the `src/bin` directory.
You can run them with `cargo run --bin <name>`. Alternatively, you can setup a
[workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

## Run clippy & cargo fmt

This should have become a habit by now. You can disable clippy warnings for a single item with `#[allow(clippy::...)]`,
but in most cases you shouldn't do that.

## If you need to escape characters in a string, use raw strings

See [https://doc.rust-lang.org/reference/tokens.html#raw-string-literals](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals)

## How to handle errors?

Short: [https://kerkour.com/rust-error-handling](https://kerkour.com/rust-error-handling)

Long: [https://www.lpalmieri.com/posts/error-handling-rust/](https://www.lpalmieri.com/posts/error-handling-rust/)

## Don't pass around locked mutex's contents

If you have a mutex, you can use `lock()` to get a guard that will unlock the mutex when it goes out of scope.
But don't pass the contents of the guard to functions that can block (unless the mutex _must_ be locked for
the entire duration of the function).
Instead of:

```rust
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn handle_function(counter: &mut i32) {
    thread::sleep(Duration::from_secs(1));
    *counter += 1;
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let counter = Mutex::new(1);

    thread::scope(|s| {
        for i in 0..10 {
            let counter = &counter;
            s.spawn(move || {
                println!("Thread {i} started");
                let now = Instant::now();
                let mut counter = counter.lock().unwrap();
                handle_function(&mut counter); // lock is held for 2 seconds
                println!("Thread {i} finished after {}s", now.elapsed().as_secs());
            });
        }
    })
}
```

You should do this:

```rust
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn handle_function(counter: &Mutex<i32>) { // <-- changed
    thread::sleep(Duration::from_secs(1));
    {
        let mut counter = counter.lock().unwrap(); // <-- changed
        *counter += 1;
        // lock is held only for the duration of the block
        // it is important to create a new scope here, otherwise the lock would be held for another second
    }
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let counter = Mutex::new(1);

    thread::scope(|s| {
        for i in 0..10 {
            let counter = &counter;
            s.spawn(move || {
                println!("Thread {i} started");
                let now = Instant::now();
                handle_function(counter); // <-- changed! we don't lock here
                println!("Thread {i} finished after {}s", now.elapsed().as_secs());
            });
        }
    })
}

```

Compare the output of the two programs. The first one will take 20 seconds to finish, while the second one will take 2 seconds.

First one:

```
Thread 1 started
Thread 0 started
Thread 2 started
Thread 3 started
Thread 4 started
Thread 5 started
Thread 6 started
Thread 7 started
Thread 8 started
Thread 9 started
Thread 1 finished after 2s
Thread 0 finished after 4s
Thread 2 finished after 6s
Thread 3 finished after 8s
Thread 4 finished after 10s
Thread 5 finished after 12s
Thread 6 finished after 14s
Thread 7 finished after 16s
Thread 8 finished after 18s
Thread 9 finished after 20s

```

Second one:

```
Thread 0 started
Thread 2 started
Thread 1 started
Thread 3 started
Thread 4 started
Thread 5 started
Thread 6 started
Thread 7 started
Thread 8 started
Thread 9 started
Thread 1 finished after 2s
Thread 2 finished after 2s
Thread 0 finished after 2s
Thread 3 finished after 2s
Thread 4 finished after 2s
Thread 5 finished after 2s
Thread 6 finished after 2s
Thread 7 finished after 2s
Thread 8 finished after 2s
Thread 9 finished after 2s
```
