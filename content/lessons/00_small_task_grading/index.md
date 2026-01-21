+++
title = "Small task grading"
date = 2026-01-18
weight = 0
[extra]
lesson_date = 2025-10-02
+++

Each entry below states a general rule (what we check when grading small tasks) followed by an explicit example.

- As long as the solution makes sense and passes all the tests, you get the maximum number of points, which then gets reduced based on the number of review comments we gave. There is no fixed convention, but usually tiny review comments give just a tiny penalty (multiples of 0.1 point).

  _Example (one reason for a small deduction)._ Bringing heavyweight dependencies or overcomplicated helpers for simple tasks usually costs you a remark.

  ```rust
  use regex::Regex;

  fn contains_rust(input: &str) -> bool {
      // Wrong: regex for a plain substring search.
      Regex::new("rust").unwrap().is_match(input)
  }

  fn contains_rust_clean(input: &str) -> bool {
      // Right: no extra crate, no unwrap.
      input.contains("rust")
  }
  ```

  _Outcome_: the former version might receive a small penalty comment even though it passes the tests.

- The solutions shouldn't have any unnecessary files or code. As long as they satisfy the task requirements, they are fine. You can just see the PR diff on GitHub before submitting.

  _Example._

  ```rust
  // src/bin/debug_experiment.rs (please remove before submitting)
  fn main() {
      dbg!(compute_answer(vec![42]));
  }
  ```

  Keep the final submission lean:

  ```rust
  // src/main.rs
  fn main() {
      println!("{}", compute_answer(read_input()));
  }
  ```

- As memory management is a big part of the Rust language, to learn how to do it optimally, the solutions should minimize the used memory (while still doing it safely). When possible, prefer to avoid allocations.

  _Example._ Borrow when you can instead of cloning everything into new structures.

  ```rust
  fn color_names_owned(palette: &[String]) -> Vec<String> {
      // Wrong: clones every string even if the caller owns the data already.
      palette.iter().cloned().collect()
  }

  fn color_names_borrowed<'a>(palette: &'a [String]) -> Vec<&'a str> {
      // Right: zero allocations, just reuses the existing storage.
      palette.iter().map(|name| name.as_str()).collect()
  }
  ```

  The same applies to arithmetic helpers—prefer in-place changes that do not allocate.

- Algorithmic complexity and the constant behind the solutions matters too. For example, prefer using single operations on `HashMap` instead of multiple operations, and prefer to use data structures that minimize lookup time.

  _Example._

  ```rust
  // Wrong: two lookups plus an unnecessary clone.
  if map.contains_key(key) {
      let value = map.get(key).unwrap().clone();
      handle(value);
  }

  // Right: one lookup, zero copies.
  if let Some(value) = map.get(key) {
      handle(value);
  }
  ```

  Choosing clever arithmetic tricks that hide overflow is another easy way to lose points, and similar "unsafe" shortcuts are treated the same way:

  ```rust
  // Wrong: overflow risk hidden in the addition.
  Color::Rgb(r, g, b) => *b > (*r + *g) / 2,

  // Wrong: casting to u16 does not fix the logic.
  Color::Rgb(r, g, b) => (*b as u16) * 3 > (*r as u16) + (*g as u16) + (*b as u16),

  // Right: the spec said "blue accounts for at least a third".
  Color::Rgb(r, g, b) => *b > *r && *b > *g,
  ```

- As the functional programming approach is also a big part of the Rust language, prefer to use it over imperative programming. Especially whenever you do operations on containers. Finding the cleanest (and usually safest) approach is left to you.

  _Example._

  ```rust
  // Wrong: manual loops, mutable counters, and temporary Vecs.
  let mut sum = 0;
  for value in numbers {
      if value % 2 == 0 {
          sum += value * value;
      }
  }

  // Right: express intent via iterator adapters.
  let sum: i32 = numbers
      .iter()
      .filter(|v| *v % 2 == 0)
      .map(|v| v * v)
      .sum();
  ```

- Error handling should be done consistently, as per lessons' guidelines. Be sure that you understand when to use `unwrap`, `expect`, `?`, `Result`. Design the code to minimize the number of assumptions and `unwrap`s.

  _Example (pattern repeats in many forms)._

  ```rust
  // Wrong: silently assumes favorite_color exists.
  if self.favorite_color.is_some() {
      self.favorite_color.as_mut().unwrap().lighten();
  }

  // Preferred: borrow only when the value is actually present.
  if let Some(ref mut color) = self.favorite_color {
      color.lighten();
  }

  if let Some(color) = &mut self.favorite_color {
      color.lighten();
  }
  ```

  The exact same advice applies to every `Option`/`Result`; any other unsafe `unwrap` is treated the same way:

  ```rust
  let x: Option<i32> = some_function();

  if x.is_some() {
      println!("x is {}", x.unwrap());
  }

  do_something(x.unwrap()); // blows up when x == None.

  if let Some(x) = x {
      println!("x is {}", x);
      do_something(x);
  }
  ```

  If a function already returns `Result`, propagate failures with `?` instead of panicking; the same rule applies to `Option` or custom error types:

  ```rust
  fn parse_duration(input: &str) -> Result<Duration, ParseIntError> {
      // Wrong: panics on invalid input.
      let seconds = input.parse::<u64>().unwrap();
      Ok(Duration::from_secs(seconds))
  }

  fn parse_duration(input: &str) -> Result<Duration, ParseIntError> {
      // Right: invalid input bubbles up to the caller.
      let seconds = input.parse::<u64>()?;
      Ok(Duration::from_secs(seconds))
  }
  ```

  You will learn about more "wrong" examples when discussing error handling.

- During lessons we do not dive into the documentation to learn all available functions from the standard library. It is your job to explore the documentation and find functions that solve whatever specific issue you have. We might give review comments whenever there's some cleaner approach using the standard library.

  _Example (a small subset of helpers from the standard library)._

  ```rust
  Color::Named(ref mut name) => *name = "light ".to_string() + name; // Wrong

  Color::Named(ref mut name) => *name = format!("light {name}"); // Right
  ```

  ```rust
  *r = (*r + 10).min(255);
  *g = (*g + 10).min(255);
  *b = (*b + 10).min(255);
  // Wrong: extra branching and assumptions.

  *r = r.saturating_add(10);
  *g = g.saturating_add(10);
  *b = b.saturating_add(10);
  // Right: std already solved it for you.
  ```

  ```rust
  let tmp = robot1.held_item.take();
  robot1.held_item = robot2.held_item.take();
  robot2.held_item = tmp; // Wrong

  mem::swap(&mut robot1.held_item, &mut robot2.held_item); // Right
  ```

  ```rust
  let tmp1: u32 = <u8 as Into<u32>>::into(*c) * 2; // Wrong

  let tmp1 = u32::from(*c) * 2;
  let tmp1 = *c as u32 * 2; // Both Right
  ```

  ```rust
  fn greet(name: &String) { println!("Hello {name}"); } // Wrong: forces callers to allocate

  fn greet(name: &str) { println!("Hello {name}"); } // Right: accepts String, &str, Cow, etc.
  ```

  _Whenever the standard library already solves the task, relying on your own helper raises similar remarks._

- We can also give review comments related to general code quality, not strictly related to the Rust language. In particular, please avoid copy-pasting, writing non-meaningful comments, overcomplicating the approach, using unoptimal data types. The approach and readability of the code also matters.

  _Example (type-system misuse)._ Let the type system encode invariants instead of checking them at runtime:

  ```rust
  impl<const N: usize> Shape for SphereN<N> {
      type Volume = VolumeN<N>;
      fn volume(&self) -> Self::Volume {
          let mut volume: u32 = (f64::from(self.radius) * f64::from(self.radius) * PI) as u32;
          if N == 3 {
              volume = (f64::from(self.radius)
                  * f64::from(self.radius)
                  * f64::from(self.radius)
                  * PI
                  * 4.0
                  / 3.0) as u32;
          }
          Self::Volume::new(volume)
      }
  }
  ```

  Instead, provide distinct impls for `SphereN<2>` and `SphereN<3>` (or use specialization) so unsupported dimensions fail to compile. Similarly, treat `u32` and `u64` as different semantic types—casting back and forth defeats the whole point of type safety.

  ```rust
  struct Volume32(u32);
  struct Volume64(u64);

  fn set_volume(v: Volume32) { /* ... */ }
  // Callers cannot accidentally provide a u64 without explicitly converting and acknowledging the loss of precision.
  ```

  Proper synchronization falls into the same category: do not hold mutex guards while doing long or blocking work. _Any excessively long critical section is penalized._

  ```rust
  fn handle_function(counter: &mut i32) {
      thread::sleep(Duration::from_secs(1));
      *counter += 1; // lock held for the entire duration
      thread::sleep(Duration::from_secs(1));
  }

  fn handle_function(counter: &Mutex<i32>) {
      thread::sleep(Duration::from_secs(1));
      {
          let mut counter = counter.lock().unwrap();
          *counter += 1; // lock is held only here
      }
      thread::sleep(Duration::from_secs(1));
  }
  ```

  The second variant lets other threads make progress and avoids a massive performance penalty.

  There are many more examples where we might subtract points.

- Lastly, of course the solutions should adhere to guidelines that are in the lessons (both in the provided text and spoken during classes).

  In particular, run `cargo fmt` and `cargo clippy` before submitting.
