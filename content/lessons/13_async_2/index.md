+++
title = "Async: Part 2"
date = 2022-05-23 
weight = 1
[extra]
lesson_date = 2024-12-05 
+++

## Reinventing futures

We recently got our feet wet with the async/await functionality of Rust by using the Tokio library. With this basic understanding of what we expect out of `futures`, let's try to come up with their details ourselves.

We know that, when asked, a future can either give us a ready value or still be waiting for it. Asking about the future's result is called _polling_. Our future could look something like this:

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

The `poll` method can be called to check for the result of the future. There is a flaw in this however - whatever is coordinating our future-based computations will have to constantly poll each of them in hope they are ready to do some work.

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

```

We can solve this by attaching a callback to our polling. The `wake` function passed to `poll` can be used to notify whoever issued the poll that the future is ready to make some progress and should be polled.

Let's picture a quick example of how our `SimpleFuture` could be used.

```rust
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // The socket has data -- read it into a buffer and return it.
            Poll::Ready(self.socket.read_buf())
        } else {
            // The socket does not yet have data.
            //
            // Arrange for `wake` to be called once data is available.
            // When data becomes available, `wake` will be called, and the
            // user of this `Future` will know to call `poll` again and
            // receive data.
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
```

### Combining futures

With the `SimpleFuture` at our disposal we can easily model more advanced concurrent computations.

```rust
/// Concurrency is achieved via the fact that calls to `poll` each future
/// may be interleaved, allowing each future to advance itself at its own pace.
pub struct Join<FutureA, FutureB> {
    // Each field may contain a future that should be run to completion.
    // If the future has already completed, the field is set to `None`.
    // This prevents us from polling a future after it has completed, which
    // would violate the contract of the `Future` trait.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // Attempt to complete future `a`.
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // Attempt to complete future `b`.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // Both futures have completed -- we can return successfully
            Poll::Ready(())
        } else {
            // One or both futures returned `Poll::Pending` and still have
            // work to do. They will call `wake()` when progress can be made.
            Poll::Pending
        }
    }
}
```

We can also queue futures like this:

```rust
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // We've completed the first future -- remove it and start on
                // the second!
                Poll::Ready(()) => self.first.take(),
                // We couldn't yet complete the first future.
                Poll::Pending => return Poll::Pending,
            };
        }
        // Now that the first future is done, attempt to complete the second.
        self.second.poll(wake)
    }
}
```

### Exercise

The last example assumes that both futures are already constructed. In practice, however, we often want to chain futures that use the results of their predecessors, like this - `get_breakfast().and_then(|food| eat(food));`. Try implementing this behavior by adding a new method to the `SimpleFuture` trait called `and_then` and something that models this sequential computation (like the previous `AndThenFut` future).

### The real deal

We weren't far from the actual way Rust's futures are structured. The `Future` trait looks as follows:

```rust
trait Future {
    type Output;
    fn poll(
        // Note the change from `&mut self` to `Pin<&mut Self>`:
        self: Pin<&mut Self>,
        // and the change from `wake: fn()` to `cx: &mut Context<'_>`:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
```

There are two differences here. Firstly, we use a context instead of a standalone `wake` method. Since this callback was just a simple function pointer, there was no way for it to hold any data pertaining to which future called it.
Secondly, we take `self` as a `Pin<>`. This enables us to create immovable futures - we will go into it later.

## Coordinating futures - waker & executor

### Using wakers and context

We will follow the [steps](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html) in the book to make a future that runs a separate thread that sleeps for a given duration and only then returns a result.

### Executor

We will follow the [steps](https://rust-lang.github.io/async-book/02_execution/04_executor.html) in the book to create our own executor to run our futures on.

## Obligatory reading

- [Pinning in detail](https://rust-lang.github.io/async-book/04_pinning/01_chapter.html)
- [Pinning in even more detail](https://doc.rust-lang.org/nightly/std/pin/index.html)
- [Async in depth](https://tokio.rs/tokio/tutorial/async)

## Additional reading

- [What color is your function](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)
- [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
- [Understanding Rust futures by going way too deep](https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep)
