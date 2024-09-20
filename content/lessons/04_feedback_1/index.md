+++
title = "Feedback #1"
date = 2022-10-31
weight = 1
[extra]
lesson_date = TODO
+++

## Feedback

### Unwrapping

Instead of this:

```rust
if self.favorite_color.is_some() {
    self.favorite_color.as_mut().unwrap().lighten();
}
```

do this:

```rust
if let Some(ref mut color) = self.favorite_color {
    color.lighten();
}
```

or

```rust
if let Some(color) = &mut self.favorite_color {
    color.lighten();
}
```

(unwrapping is a code smell)

### Spot the overflow

```rust
Color::Rgb(r, g, b) => *b > (*r + *g) / 2,
```

### 1/3

```rust
Color::Rgb(r, g, b) => (*b as u16) * 3 > (*r as u16) + (*g as u16) + (*b as u16),
```

No need to cast to u16. If b accounts for 1/3 of the sum, it's enough to check that it's bigger than both r and g.

### Format

```rust
Color::Named(ref mut name) => *name = "light ".to_string() + name,
```

There's a `format!` macro for this.

```rust
Color::Named(ref mut name) => *name = format!("light {name}"),
```

### From vs Into vs as

```rust
let tmp1: u32 = <u8 as Into<u32>>::into(*c) * 2;
```

This could be written as

```rust
let tmp1: u32 = (*c).into() * 2;
```

or even simpler (note the omission of the type annotation):

```rust
let tmp1 = u32::from(*c) * 2;
```

However in most cases of numeric conversion you can just use `as`:

```rust
let tmp1 = *c as u32 * 2;
```

[Into trait docs](https://doc.rust-lang.org/std/convert/trait.Into.html)

### Saturating addition

There's a `saturating_add` method on `u8` which does exactly what we wanted.
But it was fun watching you struggle with it :)

```rust
fn lighten(&mut self) {
    match self {
        Color::Named(name) => *name = "light ".to_string() + name,
        Color::Rgb(r, g, b) => {
            *r = r.saturating_add(10);
            *g = g.saturating_add(10);
            *b = b.saturating_add(10);
        }
    }
}
```

### Exchange

```rust
fn exchange_items(robot1: &mut Robot, robot2: &mut Robot) {
    mem::swap(&mut robot1.held_item, &mut robot2.held_item);
}
```

Swap is the preferred way to exchange the contents of two variables.

### Regex? Nope

There's no need to use a regex here. String has a `contains` method.

If you **really** want to use a regex,
you can use the `lazy_static` crate to avoid recompiling the regex every time you call the function.
