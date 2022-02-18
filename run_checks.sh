#!/bin/bash

zola check
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all --check
cargo test --all-targets --all-features --no-fail-fast
npx stylelint "**/*.{scss, css}" --fix
npx eslint "**/*.{js, ts}" --fix
npx prettier --write .