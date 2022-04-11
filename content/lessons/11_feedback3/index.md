+++
title = "Small Task Feedback #3"
date = 2022-04-11 
weight = 1
[extra]
lesson_date = 2022-04-12 
+++

## Iterators

### Too many bools

Many people implemented the InterleaveIterator like this:

```rust
pub struct InterleaveIterator<I: Iterator, J: Iterator> {
    iter1: I,
    iter2: J,
    not_finished1: bool,
    not_finished2: bool,
    turn_of_first: bool,
}
```

There's no need to use `bool`s to keep track of whether the iterators are finished. The contract of
the `Iterator` trait specifies that `next()`

> returns None when iteration is finished. Individual iterator implementations may choose to resume iteration,
> and so calling next() again may or may not eventually start returning Some(Item) again at some point.

If you want to make sure that once the iterator returns None, it will always return None, you can
use the `fuse()` method.

### or_else

```rust
impl<I, J> Iterator for InterleaveIterator<I, J>
// where etc.
{
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret_val;

        match self.next_from_a {
            true => {
                ret_val = self.i.next();
                if ret_val.is_none() {
                    ret_val = self.j.next()
                }
            }
            false => {
                ret_val = self.j.next();
                if ret_val.is_none() {
                    ret_val = self.i.next()
                }
            }
        }

        self.next_from_i = !self.next_from_i;

        ret_val
    }
}
```

Even though in this definition we don't have the excessive `bool`s,
it can still be written a lot more concisely using `or_else`.

```rust
impl<I, J> Iterator for InterleaveIterator<I, J>
// where etc.
{
    fn next(&mut self) -> Option<Self::Item> {
        self.next_from_i = !self.next_from_i;
        if self.first {
            self.i.next().or_else(|| self.j.next())
        } else {
            self.j.next().or_else(|| self.i.next())
        }
    }
}
```

### Why not `or`?

The `or` method evaluates the argument event if it's not used.
Because calling `self.i.next()` is side-effecting, that would introduce a bug.

### step_by

```rust
impl<'a> Div<usize> for Shreds<'a> {
    type Output = Shreds<'a>;

    fn div(self, rhs: usize) -> Self::Output {
        let mut counter = 0;
        let shreds = self
            .shreds
            .into_iter()
            .fold(vec![], |mut acc, el| {
                acc.push((el, acc.len()));
                acc
            })
            .filter(|(_, nr)| nr % rhs == 0)
            .map(|(el, _)| el)
            .collect();
        Shreds { shreds }
    }
}
```

Instead of `fold` we can use `enumerate` to pair each element with its index.

```rust
impl<'a> Div<usize> for Shreds<'a> {
    type Output = Shreds<'a>;

    fn div(self, rhs: usize) -> Self::Output {
        let mut counter = 0;
        let shreds = self
            .shreds
            .into_iter()
            .enumerate()
            .filter(|(_, nr)| nr % rhs == 0)
            .map(|(el, _)| el)
            .collect();
        Shreds { shreds }
    }
}
```

However, it can be simplified even more. What we're doing here is basically reimplementing
`step_by` :)

```rust
impl<'a> Div<usize> for Shreds<'a> {
    type Output = Shreds<'a>;
    fn div(self, rhs: usize) -> Self::Output {
        Shreds {
            shreds: self.shreds.into_iter().step_by(rhs).collect(),
        }
    }
}
```

```rust
impl<'a> Div<usize> for Shreds<'a> {
    type Output = Shreds<'a>;

    fn div(self, rhs: usize) -> Self::Output {
        let mut counter = 0;
        let shreds = self
            .shreds
            .into_iter()
            .enumerate()
            .filter_map(|(_, nr)| nr % rhs == 0)
            .map(|(el, _)| el)
            .collect();
        Shreds { shreds }
    }
}
```

### What's `collect()`?

It's not magic. We can collect the elements of an iterator into any type which implements
the appropriate `FromIterator` [trait](https://doc.rust-lang.org/std/iter/trait.FromIterator.html).

## Shredding usize

Instead of

```rust
impl Shredder for usize {
    fn shred(&self) -> Shreds {
        let mut elements = Vec::new();
        for i in self.to_string().chars() {
            let dig = i as usize - '0' as usize;
            let val_dig = crate::value::Digit::new(dig);
            elements.push(Value::Digit(val_dig));
        }
        Shreds { elements }
    }
}
```

it's better to use the modulo operator and division to get the digits. Why? Converting a number to
string requires an additional heap allocation.

## Make illegal states unrepresentable

Some people used an i8 or some other integer type to keep track of whose turn it is. But the only
values that were only ever used were 0 and 1. It means that there was a lot of cases where the
program would panic. Making it possible to encode an illegal state is
a [footgun](https://en.wiktionary.org/wiki/footgun). Using a `bool` would be a better choice. But
what if there are more than two states? We can define a custom enum then.
