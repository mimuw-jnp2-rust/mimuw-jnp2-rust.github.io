+++
title = "Reasoning About Types"
date = 2025-10-20
weight = 1
[extra]
lesson_date = 2025-10-23
+++

# Type traits

Traits are a way to defined common behavior between different types. They can be compared to _interfaces_ from many other mainstream languages or to type classes from Haskell, however, Rust is not an object-oriented language and there are some notable differences between type traits and Java interfaces.

The way we describe behavior in Rust is through methods. Traits consist of a set of these methods which then should be implemented by a type. We've already encountered examples of these, like the `Clone` trait which specified that the `clone()` method can be called on some given type. Now, let's take a deeper look and try defining our own trait.

{{ include_code_sample(path="lessons/05_types_reasoning/basic_trait.rs", language="rust") }}

## Default implementations

Trait definitions can also be provided with default implementations of behaviors.

{{ include_code_sample(path="lessons/05_types_reasoning/trait_default.rs", language="rust") }}

## What about _derive_?

There is a trait-related feature we have used quite extensively but not explained yet, namely the `#[derive]` attribute. When placed above a struct or enum, it tells the compiler to generate an implementation of certain traits automatically. For example, `#[derive(Debug)]` will cause the compiler to create the necessary `impl Debug for YourType { ... }` code behind the scenes, so that your type can be printed with `{:?}` in `println!`.

We'll learn about how to make it work with our own traits in the next lessons. For now, you can think of `derive` as a kind of code generator built into the compiler, which is especially useful when the implementation of a trait can be generalized for any type.

Below you can find a list of derivable traits from the standard library.

- Equality traits: `Eq`, `PartialEq` and comparison traits: `Ord` and `PartialOrd`. The `Partial-` versions exist because there are types which don't fulfill the reflexivity requirement of equality (`NaN != NaN`) or do not form a total order (` NaN < 0.0 == false` and `NaN >= 0.0 == false`).

- Data duplication traits: `Clone` and `Copy`

- `Hash` - allows using values of that type as keys in a hashmap

- `Default` - provides a zero-arg constructor function

- `Debug` - provides a formatting of the value which can be used in debugging context. Because the `derive` attribute automatically implements a pretty way of formatting, it is discouraged to implement this trait manually. In general, if it's possible to derive the `Debug`, there are no reasons against doing it.

### When is it possible to derive a trait?

When all fields of a struct/variants of an enum implement that trait.

### Should all traits always be derived if it is possible?

No. Although it may be tempting to just slap `#[derive(Clone, Copy)]` everywhere, it would be counter-effective. For example, at some later point you might add a non-`Copy` field to the struct and your (or, what's worse, someone else's!) code would break. Another example: it makes little sense to use containers as keys in hashmaps or to compare tweets.

# Generics

Suppose we want to find the largest element in a sequence and return it. Very much on purpose, we didn't specify what type these elements would be - ideally, we would love it to work on all types that have a defined notion of a _largest_ element. However, to make things simpler for now, let's focus only on two primitive types: `i32` and `char`. Let's try to write the code:

{{ include_code_sample(path="lessons/05_types_reasoning/non_generic.rs", language="rust") }}

Perfect, it works! Now only twenty more types to go...

Of course, Rust gives us a way to avoid all this code duplication and generalize the types we're working on.

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

Cleaner already - we merged possibly very many implementations into one. But, when we try to compile this:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ++++++++++++++++++++++
```

Since `T` can be of absolutely any type now, the compiler cannot be sure that operator `>` is defined. This aligns with what we wanted, as without comparing elements we don't have a notion of the largest one either. As always, the compiler messages comes to our aid:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

We call this a _trait bound_, a way to provide constraints on what kind of types we are talking about in a given context. This implementation almost works now. Let's look at the new error.

```
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`
```

Our function attempts to take ownership, but, again, the compiler doesn't know whether `T` can just be trivially copied. Rust allows us to combine multiple trait bounds together:

{{ include_code_sample(path="lessons/05_types_reasoning/generic_largest.rs", language="rust") }}

## A powerful tool

There's a lot more that we can do with generics:

{{ include_code_sample(path="lessons/05_types_reasoning/generics.rs", language="rust") }}

An example where we can specify which generic trait implementation we want to call:

{{ include_code_sample(path="lessons/05_types_reasoning/generics_fun.rs", language="rust") }}

## Static vs dynamic dispatch

{{ include_code_sample(path="lessons/05_types_reasoning/static_dynamic_dispatch.rs", language="rust") }}

# Lifetimes

Let's go into a completely different topic now.
Going back to the lesson about ownership, if we try to compile the following code:

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

we should expect to get an error:

```
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  |
10 |         println!("r: {}", r);
   |                           - borrow later used here
```

Courtesy of the borrow checker, we didn't end up with a dangling reference. But what exactly is happening behind the scenes? Rust introduces a concept of annotated lifetimes, where the lifetime of each value is being marked and tracked by the checker. Let's look at some examples:

```rust
{
    let r;                  // ---------+-- 'a
                            //          |
    {                       //          |
        let x = 5;          // -+-- 'b  |
        r = &x;             //  |       |
    }                       // -+       |
                            //          |
    println!("r: {}", r);   //          |
}                           // ---------+
```

```rust
{
    let x = 5;              // ----------+-- 'b
                            //           |
    let r = &x;             // --+-- 'a  |
                            //   |       |
    println!("r: {}", r);   //   |       |
                            // --+       |
}                           // ----------+
```

## Annotations

Let's consider the following code finding the longer out of two strings:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

If we try to compile this, we will get an error:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

This is because Rust doesn't know which of the two provided strings (`x` or `y`) will be returned from the function. And because they potentially have different lifetimes, the lifetime of what we are returning remains unclear to the compiler - it needs our help.

Rust provides syntax for specifying lifetimes. The lifetime parameter name from the example (`a`) doesn't have any concrete meaning - it's just an arbitrary name for this one lifetime.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

So, knowing this, let's address the compiler's demands.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

When working with lifetimes, our work will usually revolve around specifying relationships between lifetimes of different values so that the compiler can successfully reason about the program's safety. In the context of the example above, this signature means that both of the function's arguments and its output will live at least as long as lifetime `'a`. In practice, this means that the output's lifetime will be equal to the smaller of the two inputs' lifetimes.

{{ include_code_sample(path="lessons/05_types_reasoning/lifetimes_basic.rs", language="rust") }}

Trying to compile the second variant displeases the compiler (just like we hoped).

```
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

## Lifetime elision

We now know how to explicitly write lifetime parameters, but you might recall that we don't always have to do that. Indeed, Rust will first try to figure out the lifetimes itself, applying a set of predefined rules. We call this _lifetime elision_.

{{ include_code_sample(path="lessons/05_types_reasoning/lifetimes_elision.rs", language="rust") }}

The above works, even though we didn't specify any lifetime parameters at all. The reason lies in the rules we mentioned, which are as follows (where input lifetimes are lifetimes on parameters and output lifetimes are lifetimes on return values):

- Each parameter that is a reference gets its own lifetime parameter.

- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.

- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

Let's try to understand how the compiler inferred the lifetimes of our `first_two` functions. We start with the following signature:

```rust
fn first_two(seq: &[u32]) -> &[u32] {
```

Then, we apply the first rule:

```rust
fn first_two<'a>(seq: &'a [u32]) -> &[u32] {
```

Next, we check the second rule. It applies here as well.

```rust
fn first_two<'a>(seq: &'a [u32]) -> &'a [u32] {
```

With that, we arrive at a state where all lifetimes are specified.

## Static lifetime

There exists one special lifetime called `'static`, which means that a reference can live for the entire duration of the program. All string literals are annotated with this lifetime as they are stored directly in the program's binary. Full type annotation of a string literal in Rust is therefore as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```

# Using `Display` trait

Let's look again into our `summarize` function from the beginning of the lesson. We can use it to print the summary by doing `println!("1 new article: {}", news_article.summarize());`.

But because we have implemented it in this way:

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

we can see that we do one unnecessary heap allocation that we can optimize. Instead of creating a `String` (which does the heap allocation) that we then pass to `format!`, we can just use `format!` directly:

```rust
impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )
    }
}
```

then we can simply call `println!("1 new article: {}", news_article);`. This is quite simple after we get acclimated to the interface of the `Display` trait.

### To discuss during class:

- But what if we want to be able to display the article in two ways, as a summary or with its full content? How to solve this issue in Python and in C++?
- How to solve it in Rust? Discuss multiple approaches.


## Associated types vs `impl Trait`

Rust offers two complementary tools for expressing "some type that implements a trait":

- **Associated types** belong to a trait definition. Each implementer fills in the concrete type once, driving a consistent contract across all of the trait's methods. Use them when the trait itself needs to name a type (for example, an iterator's `Item` or a parser's `Output`). They shine when multiple methods must agree on the exact type, or when a caller should be able to refer to it explicitly (`MyIterator::Item`).

```rust
trait Parser {
    type Output;

    fn parse(&self, input: &str) -> Self::Output;
}

struct BoolParser;

impl Parser for BoolParser {
    type Output = bool;

    fn parse(&self, input: &str) -> bool {
        input.eq_ignore_ascii_case("true")
    }
}

fn collect_truths<P: Parser<Output = bool>>(parser: P, inputs: &[&str]) -> Vec<bool> {
    inputs.iter().map(|line| parser.parse(line)).collect()
}
```

The trait owns the `Output` type name, so every helper that works with `Parser` can refer to `Parser::Output` without re-listing the concrete type parameter everywhere.

- **`impl Trait`** appears in function signatures. It hides a concrete type from callers while promising that "this value implements trait `T`". Reach for it when returning helper types (iterators, closures) or when you want a lightweight way to accept any type satisfying a bound without introducing a type parameter at the call site. Remember that a single function returning `impl Trait` must always produce the same concrete type on every path.

```rust
fn doubled_evens<'a>(values: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    values.iter().copied().filter(|n| n % 2 == 0).map(|n| n * 2)
}

fn show_evens(values: &[i32]) {
    for value in doubled_evens(values) {
        println!("{value}");
    }
}
```

Here the caller only cares that the iterator implements `Iterator<Item = i32>`, not about the exact adapter stack we built. As long as every branch returns the same iterator type, the function can keep its helper type private.

In short: if the abstraction lives in a trait and needs a name the implementer controls, choose an associated type; if you are exposing a function and simply want to promise trait behavior without leaking the concrete type, reach for `impl Trait`.

# Obligatory reading

- [The Book, chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html)

- [Polymorphism in Rust](https://oswalt.dev/2021/06/polymorphism-in-rust/)

## Assignment 3 (graded)

[Passage Pathing TODO CHANGE LINK](https://classroom.github.com/a/VTyPdlC2)

Deadline: per-group.
