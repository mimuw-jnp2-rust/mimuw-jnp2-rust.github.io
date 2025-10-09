+++
title = "Ownership Model"
date = 2025-10-08
weight = 1
[extra]
lesson_date = 2025-10-09
+++

## Why all the fuss?

Even if you've never seen Rust code before, chances are you still heard the term _borrow checker_ or something about Rust's ownership. Indeed, Rust's ownership model lies at the very core of its uniqueness. But to fully understand it and appreciate it, let's first take a look at how memory management is handled in most popular languages.

- **Garbage Collection** - in many high-level programming languages, like Java, Haskell or Python, memory management is done fully by the language, relieving the programmer from this burden. This prevents memory leaks and memory related errors (like _use after free_), but does come at a cost - there is a runtime overhead, both memory and performance wise, caused by the constantly running garbage collection algorithms and the programmer usually has very little control over when the garbage collection takes place. Also, garbage collection does not prevent concurrency-related errors, such as data races, in any way.

  There's one language that has a lot of common parts with Rust, but it differs by having a garbage collector: OCaml. In Jane Street, programmers heavily use OCaml as their primary language, and the drawbacks brought by garbage collectors are visible. Their programmers decided to extend the OCaml compiler to be able to write code without any allocations (this approach is quite restrictive) to avoid calling the garbage collector, just so their code runs faster. That's quite a roundabout way!

- **Mind your own memory** - in low-level languages and specific ones like C++, performance comes first so we cannot really afford to run expansive bookkeeping and cleaning algorithms. Most of these languages compile directly to machine code and have no language-specific runtime environment. That means that the only place where memory management can happen is in the produced code. While compilers insert these construction and destruction calls for stack allocated memory, it generally requires a lot of discipline from the programmer to adhere to good practices and patterns to avoid as many memory related issues as possible and one such bug can be quite deadly to the program and a nightmare to find and fix. These languages basically live by the _"your memory, your problem"_ mantra.

  For example, in C++ we get some help through variable scopes, but whenever we write a function, we have to decide ourselves how to pass the values. It's easy by mistake to copy a value, pass an invalid pointer or use some dangling reference (i.e., one pointing to an already-freed data).

## To discuss during class

- Why languages with garbage collectors are often considered slower? What different garbage collector mechanisms are there?
- Why coding without allocations can avoid using a garbage collector?
- Besides variable scope, does C++ help in any other way with memory management?

## Start with the basics - ownership

And then we have Rust. Rust is a systems programming language and in many ways it's akin to C++ - it's basically low-level with many high-level additions. But unlike C++, it doesn't exactly fall into either of the categories described above, though it's way closer to the second one. It performs no additional management at runtime, but instead imposes a set of rules on the code, making it easier to reason about and thus check for its safety and correctness at compile time - these rules make up Rust's **ownership model**.

In a way, programming in Rust is like pair-programming with a patient and very experienced partner. Rust's compiler will make sure you follow all the good patterns and practices (by having them ingrained in the language itself) and very often even tell you how to fix the issues it finds.

_**Disclaimer:** when delving deeper into Rust below we will make heavy use of concepts like scopes, moving data, stack and heap, which should have been introduced as part of the C++ course. If you need a refresher of any of these, it's best to do so now, before reading further._

In the paragraph above we mentioned a set of rules that comprise Rust's ownership model. [The book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules) starts off with the following three as its very foundation:

1. Each value in Rust is tied to a specific variable - we call that variable its **owner**.

2. There can only be one owner at a time.

3. When the owner goes out of scope, the value will be destroyed (or in Rust terms - _dropped_).

The third point might make you think about C++ and its automatic storage duration. We will later see that, while very similar at first, Rust expands on these mechanics quite a bit. The following code illustrates the basic version of this:

```rust
{
    // Allocation on the stack, 'a' becomes an owner.
    let a: i32 = 5;

    // Do some stuff with 'a'.

}
// 'a', the owner, goes out of scope and the value is dropped.
```

So far, so good. Variables are pushed onto the stack when they enter the scope and destroyed during stack unwinding that happens upon leaving their scope. However, allocating and deallocating simple integers doesn't impress anybody. Let's try something more complex:

```rust
{
    // 's' is allocated on the stack, while its contents ("a string")
    // are allocated on the heap. 's' is the owner of this String object.
    let s = String::from("a string");

    // do some stuff with 's'
}
// 's', the owner, goes out of scope and the String is dropped, its heap allocated memory freed.
```

If you recall the RAII (Resource Acquisition Is Initialization) pattern from C++, the above is basically the same thing. We go two for two now in the similarity department, so... is Rust really any different then? There is a part of these examples that we skipped over - actually doing something with the values.

## Moving around is fun

Let's expand on the last example. The scoping is not really important for that one, so we don't include it here.

```rust
// Same thing, 's' is now an owner.
let s = String::from("a string");

// Easy, 's2' becomes another owner... right?
let s2 = s;

// This doesn't work, can you guess why?
println!("And the contents are: {}", s);
```

At first glance everything looks great. If we write this code (well, an equivalent of it) in basically any other popular language, it will compile without issues - but it does _not_ compile here and there's a good reason why.

To understand what's happening, we have to consult the rules again, rule 2 in particular. It says that there can only be one owner of any value at a given time. So, `s` and `s2` cannot own the same object. Okay, makes sense, but what is happening in this line then - `let s2 = s;`? Experience probably tells you that `s` just gets copied into `s2`, creating a new String object. That would result in each variable owning its very own instance of the string and each instance having exactly one owner. Sounds like everyone should be happy now, but wait - in that case the last line should work no issue, right? But it doesn't, so can't be a copy. Let's see now what the compiler actually has to say:

```
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:6:42
  |
2 |     let s = String::from("a string");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |
4 |     let s2 = s;
  |              - value moved here
5 |
6 |     println!("And the contents are: {}", s);
  |                                          ^ value borrowed here after move
```

_"value moved here"_ - gotcha! So `s` is being moved to `s2`, which also means that `s2` now becomes the new owner of the string being moved and `s` cannot be used anymore. In Rust, the default method of passing values around is by move, not by copy. While it may sound a bit odd at first, it actually has some very interesting implications. But before we get to them, let's fix our code, so it compiles now. To do so, we have to explicitly tell Rust to make a copy by invoking the `clone` method:

```rust
let s = String::from("a string"); // 's' is an owner.

let s2 = s.clone(); // 's2' now contains its own copy.

println!("And the contents are: {}", s); // Success!
```

The compiler is happy now and so are we. The implicit move takes some getting used to, but the compiler is here to help us. Now, let's put the good, old C++ on the table again and compare the two lines:

<div style="text-align: center">

`let s2 = s;` is equivalent to `auto s2 = std::move(s);`

`let s2 = s.clone()` is equivalent to `auto s2 = s`

</div>

There are a few important things to note here:

- Making a copy is oftentimes not cheap. Memory needs to be allocated and copied, and a call to the system has to be made. We should prefer to move things as much as possible to avoid this cost - in C++ we have a myriad of language features like `std::move` and _r-references_ to achieve this. Every programmer worth their salt needs to be well versed in all of them to write efficient C++ code and simply forgetting one move can lead to significant performance loss (and this happens to even the most senior devs ever existing, let's not pretend). On the contrary, in Rust you need to make an effort to make a copy and that makes you very aware of the cost you're paying - something that we'll see quite a lot of in the language. Also, if you forget a clone there's no harm done - it just won't compile!

- Hidden in all of this is another nice thing Rust gives us. In C++, nothing prevents you from using variables after they've been moved from, leading to unexpected errors in a more complex code. In Rust, that variable (in our case `s`) simply becomes invalid and the compiler gives us a nice error about it.

### But what about ints?

A good question to ask. Copying primitives is cheap. And it's not convenient for the programmer to have to always write `.clone()` after every primitive. If we take a look at the error from the previous example:

```
move occurs because `s` has type `String`, which does not implement the `Copy` trait`
```

It says that `s` was moved because the `String` type doesn't have the `Copy` trait. We will talk about traits more in depth in the future lessons, but what this basically means is that `String` is not specified to be copied by default. All primitive types (`i32`, `bool`, `f64`, `char`, etc.) and tuples consisting only of primitive types implement the `Copy` trait.

### Exercise

How to fix that code? Don't worry about efficiency yet.

```rust
fn count_animals(num: u32, animal: String) {
    println!("{} {} ...", num, animal);
}

fn main() {
  let s = String::from("sheep");

  count_animals(1, s);
  count_animals(2, s);
  count_animals(3, s);
}
```

## To discuss during class

- How does `std::move` work in C++? When can l-value references (`&`) be used, and when r-value references (`&&`)?
- What exactly are the performance benefits in copying primitives?

## Let's borrow some books

We now know how to move things around and how to clone them if moving is not possible. But what if making a copy is unnecessary - maybe we just want to let someone look at our resource while still holding that value after they're finished. Kind of like references in C++ (but with some major differences). Consider the following example:

```rust
fn read_book(book: String) {
    println!("[Reading] {}", book);
}

fn main() {
  let book = String::from("Merry lived in a big old house. The end.");

  read_book(book.clone());

  println!("Book is still there: {}", book);
}
```

Cloning is pretty excessive here. Imagine recommending a book to your friend and instead of lending it to them for the weekend, you scan it and print an exact copy. Not the best way to go about it, is it? Thankfully, Rust allows us to access a resource without becoming an owner through the use of references and the `&` operator. This is called a borrow.

The adjusted code should look like this:

```rust
fn read_book(book: &String) {
    println!("[Reading] {}", book);
}

fn main() {
  let book = String::from("Merry lived in a big old house. The end.");

  read_book(&book);

  println!("Book is still there: {}", book);
}
```

As with everything, references are too, by default, immutable, which means that the `read_book` function is not able to modify that book passed into it. We can also borrow something mutably by specifying it both in the receiving function signature and the place it gets called. Maybe you want to have your book signed by its author?

```rust
fn sign_book(book: &mut String) {
    book.push_str(" ~ Arthur Author");
}

fn main() {
  // Note that the book has to be marked as mutable in the first place.
  let mut book = String::from("Merry lived in a big old house. The end.");

  sign_book(&mut book); // It's always clear when a parameter might get modified.

  println!("{}", book); // Book is now signed.
}
```

Pretty neat, but doesn't seem that safe right now. Let's try to surprise our friend:

```rust
fn erase_book(book: &mut String) {
    book.clear();
}

fn read_book(book: &String) {
    println!("[Reading] {}", book);
}

fn main() {
  let mut book = String::from("Merry lived in a big old house. The end.");

  let r = &book; // An immutable borrow.

  erase_book(&mut book); // A mutable borrow.

  read_book(r); // Would be pretty sad to open a blank book when it was not
                // what we borrowed initially.

  println!("{}", book);
}
```

Fortunately for us (and our poor friend just wanting to read), the compiler steps in and doesn't let us do that, printing the following message:

```
error[E0502]: cannot borrow `book` as mutable because it is also borrowed as immutable
  --> src/main.rs:14:14
   |
12 |   let r = &book; // An immutable borrow.
   |           ----- immutable borrow occurs here
13 |
14 |   erase_book(&mut book); // A mutable borrow.
   |              ^^^^^^^^^ mutable borrow occurs here
15 |
16 |   read_book(r); // Would be pretty sad to open a blank book when it was not
   |             - immutable borrow later used here
```

This is where the famous borrow checker comes in. To keep things super safe, Rust clearly states what can and cannot be done with references and tracks their lifetimes. Exactly one of the following is always true for references to a given resource:

- There exists only one mutable reference and no immutable references, **or**

- There is any number of immutable references and no mutable ones.

Those rules are the core of the borrow checker. Be sure that you understand it.

You may notice a parallel to the _readers - writers_ problem from concurrent programming. Because of that, the way Rust's borrow checker is designed lends itself incredibly well to preventing data race related issues.

### Dangling references

Rust also checks for dangling references. If we try to compile the following code:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

we will get an adequate error:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ^^^^^^^^
```

The message above suggests specifing a lifetime for the returned string. In Rust, the lifetime of each variable is also a part of its type, but we will talk more about it later.

### Exercise

Our previous solution using `clone()` was pretty inefficient. How should you modify this code now?

```rust
fn count_animals(num: u32, animal: String) {
    println!("{} {} ...", num, animal);
}

fn main() {
  let s = String::from("sheep");

  count_animals(1, s.clone());
  count_animals(2, s.clone());
  count_animals(3, s); // We previously could've omitted the clone() here. Why?
}
```

## Everyone gets a slice

The last part of working with references that we will cover in this lesson are slices. A _slice_ in Rust is a view over continuous data. Let us start with a string slice - the `&str` type.

_**Note:** for the purposes of these examples we assume we are working with ASCII strings. More comprehensive articles on handling strings are linked at the end of this lesson._

To create a string slice from the `String` object `s`, we can simply write:

```rust
// Creates a slice of length 2, starting with the character at index 1.
let slice = &s[1..3];
```

This makes use of the `&` operator and Rust's range notation (analogous to Python's notation) to specify the beginning and end of the slice. Thus, we can also write:

```rust
let slice = &s[2..];    // Everything from index 2 till the end.
let slice = &s[..1];    // From beginning to first byte (so, only the first byte).
let slice = &s[..];     // The whole string as a slice.
let slice = s.as_str(); // Also the whole string.
```

You might have noticed that we always built `String` values using the `from()` method and never actually used the string literals directly. What type is a string literal then? Turns out it's the new string slice we just learned about!

```rust
let slice: &str = "string literal";
```

In fact, it makes a lot sense - string literals, after all, are not allocated on the heap, but rather placed in a special section of the resulting binary. It's only natural we just reference that place with a slice.

Slices can also be taken from arrays:

```rust
let array: [i32; 4] = [42, 10, 5, 2]; // Creates an array of four 32 bit integers.
let slice: &[i32] = &array[1..3];     // Results in a slice [10, 5].
```

### Exercise

Can this code be further modified utilizing references? Think about the signature of `count_animals`, can we make it also accept string literals?

```rust
fn count_animals(num: u32, animal: &String) {
    println!("{} {} ...", num, animal);
}

fn main() {
  let s = String::from("sheep");

  count_animals(1, &s);
  count_animals(2, &s);
  count_animals(3, &s);
  count_animals(4, "goat"); // In this version of the code it doesn't compile.
}
```

### Obligatory reading

- [The Book, chapter 4](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)

- [Rust by Example, chapter Scoping rules](https://doc.rust-lang.org/stable/rust-by-example/index.html)

### Additional reading

- [Char documentation](https://doc.rust-lang.org/std/primitive.char.html)

- [Working with strings in Rust](https://fasterthanli.me/articles/working-with-strings-in-rust)

### Assignment 1 (graded)

[ordering in Van Binh](https://classroom.github.com/a/ih0mIzmM)

Deadline: group-specific.

Grading:

- 2 points for passing tests, `cargo fmt` and `cargo clippy`,
- 1 point for manual code inspection ("code quality").
