+++
title = "Smart Pointers"
date = 2022-11-21
weight = 1
[extra]
lesson_date = 2024-11-07
+++

# Working with the heap

So far we've only used heap allocated memory indirectly by working with containers such as vectors, maps or the `String` type, otherwise allocating our variables on the stack. We didn't really have to be aware of the fact that these collections used the heap, as all that memory management details were hidden away from us. In this lesson we'll take a closer look at what is really happening there and how we can do that ourselves.

To work with heap-allocated memory, Rust features _smart pointers_. You should have already heard this term as it is a very important feature in C++ and the concept is virtually the same here - they are wrappers around raw allocated memory that provide additional, safety-ensuring mechanism. What defines a smart pointer in Rust is generally the implementation of two traits: `Drop` and `Deref`.

The `Drop` trait is pretty straightforward as it consists of one method - `fn drop(&mut self)` - that is, basically, the destructor, invoked during stack unwinding.

The `Deref` trait allows us to overload the dereference (`*`) operator.

## Deref coercion

Apart from enabling access to the underlying value, implementing the `Deref` trait enables Rust to perform _deref coercion_ on the pointer - trying to remove as many levels of indirection as it can. What it means in practice is that we will be able to use it with any code working on plain references.

{{ include_code_sample(path="lessons/07_smart_pointers/deref_coercion.rs", language="rust") }}

In general, there are three possible coercions that Rust can perform:

- From `&T` to `&U` when `T: Deref<Target=U>`

- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`

- From `&mut T` to `&U` when `T: Deref<Target=U>`

While the first two coercions are straightforward, the third one is possible because treating a mutable reference as an immutable one does not break the rules of ownership.

# `Box` - simple wrapper

The `Box<T>` type is the most basic out of Rust's smart pointers, equivalent to C++'s `std::unique_ptr<T>`. It's a simple wrapper that makes sure the underlying memory gets allocated and freed properly.

{{ include_code_sample(path="lessons/07_smart_pointers/box.rs", language="rust") }}

# Reference counting

The `Rc<T>` type is the equivalent of `std::shared_ptr<T>` from C++. There is one caveat to this though - because we're creating multiple references to the same object, those references have to be immutable in accordance with the ownership rules.

{{ include_code_sample(path="lessons/07_smart_pointers/ref_count.rs", language="rust") }}

Rust also provides a non-owning pointer in the form of `Weak<T>` (equivalent to `std::weak_ptr<T>`) that can be obtained from an instance of `Rc<T>`.

{{ include_code_sample(path="lessons/07_smart_pointers/weak_ref.rs", language="rust") }}

# Mutating the immutable

Good examples and explanation of the interior mutability pattern and runtime borrow checking can be found in the [book](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html).

Alongside the `RefCell<T>` type described above, there is an analogous [`Cell<T>`](https://doc.rust-lang.org/std/cell/struct.Cell.html) type that operates on values instead of references.

# Convenient handling of `dyn` objects

In previous labs you learned about dynamic dispatch and its strengths. The largest drawback you noticed is most likely that they are _unsized_ (`!Sized`, where `!` being syntax signifying lack of trait implementation).

When storing an object on a heap, however, we can use it as a `dyn` object seamlessly.

# Obligatory reading

- [The Book, chapter 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

- [std::borrow::Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html), a versatile copy-on-write smart pointer

# Additional reading

- [On wrapped references](https://www.fpcomplete.com/blog/rust-asref-asderef/)
- [`Deref` vs `AsRef` vs `Borrow`](https://dev.to/zhanghandong/rust-concept-clarification-deref-vs-asref-vs-borrow-vs-cow-13g6)

## Assignment 5 (graded)

[Corporations](https://classroom.github.com/a/QlO3aCCP)

Deadline: 13.11.2024 23:59
