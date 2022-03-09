+++
title = "Enums"
date = 2022-03-14
weight = 1
[extra]
lesson_date = 2022-03-09
+++

## Enums

It is often the case that you want to define a variable that can only take
a certain set of values and the values are known up front. In C you can an `enum` for this.

{{ include_code_sample(path="lessons/4_enums/enums.c", language="c") }}

However, in C enums are just integers. Nothing prevents us from writing

```c
int main() {
    enum shirt_size my_size = 666;
    print_size(my_size);
}
```

In C++ enums are more type-safe:
```
<source>:27:31: error: invalid conversion from 'int' to 'shirt_size' [-fpermissive]
   27 |     enum shirt_size my_size = 666;
      |                               ^~~
      |                               |
      |                               int
```

Some programming languages (especially functional ones) allow programmers to define
enums which carry additional information. Such types are usually called `tagged unions`
or `algebraic data types`.

In C++ we can use `union` with an `enum` tag to define it:

{{ include_code_sample(path="lessons/4_enums/tagged_union.cpp", language="cpp") }}

C++17 introduced a new feature called `variant` which generalizes this concept.
You can read more about it [here](https://en.cppreference.com/w/cpp/utility/variant).

Java has a more or less analogous feature called `sealed classes` 
since [version 17](https://docs.oracle.com/en/java/javase/17/language/sealed-classes-and-interfaces.html.).

## Rust

Let's see how they are defined in Rust.

{{ include_code_sample(path="lessons/4_enums/enums.rs", language="rust") }}

In Rust, enums are a core feature of the language. 
You may have heard that one of Rust's defining characteristics is 
the absence of ["the billion dollar mistake"](https://en.wikipedia.org/wiki/Tony_Hoare#Apologies_and_retractions).
So what can we do to say that a value is missing if there is no `null`?

In Rust, we can use the `Option` type to represent the absence of a value.

Option is defined as:
```rust
enum Option<T> {
    Some(T),
    None,
}
```
The `<T>` part is called the "type parameter" and it causes Option to be generic. 
We won't go deeper into this for now.

The fact that variables which could be `null` in other languages have a different type in Rust is
the solution to the billion dollar mistake!

{{ include_code_sample(path="lessons/4_enums/option.rs", language="rust") }}


