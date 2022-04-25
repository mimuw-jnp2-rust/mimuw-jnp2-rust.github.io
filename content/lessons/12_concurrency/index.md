+++
title = "Fearless concurrency"
date = 2022-04-25
weight = 1
[extra]
lesson_date = 2022-04-26
+++

## Parallelism vs Concurrency

Concurrency is when tasks **can make** progress **independently** of each other.

Parallelism is when multiple tasks **make** progress **at the same time**.

## Concurrency models in Rust

### Threads

Nothing unusual here.

Threads can be created with the `thread::spawn` function [docs - please read them!](https://doc.rust-lang.org/std/thread/fn.spawn.html).

This method returns a `JoinHandle<T>` which can be used to wait for the thread to finish. `T` is the type of the thread's return value.

#### Propagating panics

In Rust a panic of one thread doesn't affect the other threads (similar to how Java handles exceptions in threads).

#### Closures

Closures which are used to create threads must take ownership of any values they use. It can be forced with the `move` keyword.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

Normal ownership rules still apply. It means that we cannot mutate the vector in the spawned thread from the main thread!

But what if we need to share some state?

### Message passing

One possible way is to use message passing. We can use a blocking queue (called `mpsc` - ["multi producer single consumer FIFO queue"](https://doc.rust-lang.org/std/sync/mpsc/index.html)) to do it.
We talked about blocking queues in the Concurrent programming class. In Rust, they are strongly-typed. Sending and receiving ends have different types.

### Mutexes

In Rust, a mutex _wraps_ a value and makes it thread-safe.
Because it becomes a part of the type, it's impossible to access the underlying value in an unsynchronized manner. It is conceptually similar to the `RefCell` type.

`Arc` is a smart pointer like `Rc` but it can be shared between threads.

Please read more about them in [the book](https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html).

[The docs](https://doc.rust-lang.org/std/sync/struct.Mutex.html) also mention `poisoning`.

### RwLocks

[RwLocks](https://doc.rust-lang.org/std/sync/struct.RwLock.html) are similar to mutexes, but they distinguish between read and write locks.

## Send and Sync

They are marker traits used to indicate that a type or a reference to it can be sent across threads. See the [nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html) for more information.

## Atomic types

Atomic types are described in [the docs](https://doc.rust-lang.org/std/sync/atomic/).

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
```

Note that `atomic` values don't have to be wrapped in a mutex when shared across threads.

### Wait...

If most types are `Sync + Send`, then what stops us from using a standard, non-atomic integer in the example above?

```rust
let spinlock = Arc::new(1);

let spinlock_clone = Arc::clone(&spinlock);
let thread = thread::spawn(move|| {
    *spinlock_clone += 1;
});

while *spinlock != 0 {
    hint::spin_loop();
}
```

```
error[E0594]: cannot assign to data in an `Arc`
 --> src/main.rs:9:9
  |
9 |         *spinlock_clone += 1;
  |         ^^^^^^^^^^^^^^^^^^^^ cannot assign
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<i32>`
```

...so we would have to use a `RefCell` to be able to modify the value through a shared reference...

```rust
let spinlock = Arc::new(RefCell::new(1));

let spinlock_clone = Arc::clone(&spinlock);
let thread = thread::spawn(move|| {
    *spinlock_clone.borrow_mut() += 1;
});

// Wait for the other thread to release the lock
while *spinlock.borrow() != 0 {
    hint::spin_loop();
}
```

...but `RefCell` isn't `Sync`:

```
error[E0277]: `RefCell<i32>` cannot be shared between threads safely
   --> src/main.rs:9:18
    |
9   |     let thread = thread::spawn(move|| {
    |                  ^^^^^^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
    |
    = help: the trait `Sync` is not implemented for `RefCell<i32>`
    = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
    = note: required because it appears within the type `[closure@src/main.rs:9:32: 11:6]`
note: required by a bound in `spawn`
```

And that bound mentioned in the last line looks like this:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```

#### Exercise for the reader

Why is it impossible to share a reference to a `Mutex` between threads?

## Data parallelism with Rayon

[Rayon](https://docs.rs/rayon/latest/rayon/) is a library for parallelization of data processing.
It can be used to parallelize the execution of functions over a collection of data by switching the standard `Iterator` to a `ParallelIterator`.
It works very similar to [Java's parallel streams](https://docs.oracle.com/javase/tutorial/collections/streams/parallelism.html#executing_streams_in_parallel).

Why do that? Because thread synchronization is hard! [Rust prevents data races](https://doc.rust-lang.org/nomicon/races.html), but [logical races and deadlocks are impossible to prevent!](https://users.rust-lang.org/t/deadlock-is-it-a-bug-or-is-it-intentional/1544)!

[Rayon's FAQ](https://github.com/rayon-rs/rayon/blob/master/FAQ.md) is worth reading.
