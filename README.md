# JNP 2: Rust

This repository uses [Zola](https://www.getzola.org/documentation/getting-started/installation/) to generate static websites. Requires zola version 0.15.3.

Place new lessons in the `content` directory.

Before making a PR, please run the following commands:

```sh
zola check
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo test --all-targets --all-features --no-fail-fast
npx stylelint "**/*.{scss, css}" --fix
npx eslint "**/*.{js, ts}" --fix
npx prettier --write .
```

## Useful links

* [Rust Playground](git@github.com:mimuw-jnp2-rust/mimuw-jnp2-rust.github.io.git)

* [Rust Book](https://doc.rust-lang.org/book/title-page.html)

* [Rust Async Book](https://rust-lang.github.io/async-book/)