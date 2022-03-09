# JNP 2: Rust

This repository uses [Zola](https://www.getzola.org/documentation/getting-started/installation/) to generate static websites. Requires zola version 0.15.3.

Place new lessons in the `content` directory.

Before making a PR, please run the following commands:

```sh
zola check
cargo clippy --all-targets --all-features --fix -- -D warnings
cargo fmt --all
cargo test --all-targets --all-features --no-fail-fast
npx stylelint "**/*.{scss, css}" --fix
npx eslint "**/*.{js, ts}" --fix
npx prettier --write .
```
