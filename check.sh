#!/bin/bash

# Check if the required software is installed
function check_installed() {
    if ! [ -x "$(command -v $1)" ]; then
        echo "Error: $1 is not installed." >&2
        exit 1
    fi
}

check_installed "zola"
check_installed "cargo"
check_installed "stylelint"
check_installed "eslint"
check_installed "prettier"

# Check if zola has the version specified in zola_version file
zola_version=$(zola --version)
required_version=$(cat zola_version)
if [ "$zola_version" != "$required_version" ]; then
  echo "Error: zola version is not $required_version" >&2
  exit 1
fi

# Run formatters
echo
echo "RUSTFMT"
cargo fmt --all

echo
echo "PRETTIER"
prettier --write .

# Run checks
echo
echo "ZOLA"
zola check
ZOLA_SUCCESS=$?

echo
echo "CLIPPY"
cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged -- -D warnings
CLIPPY_SUCCESS=$?

echo
echo "TEST"
cargo test --all-targets --all-features --no-fail-fast
TEST_SUCCESS=$?

echo
echo "STYLELINT"
stylelint "**/*.{scss, css}" --fix
STYLELINT_SUCCESS=$?

echo
echo "ESLINT"
eslint "**/*.{js, ts}" --fix
ESLINT_SUCCESS=$?

SUCCESS=("$ZOLA_SUCCESS" "$CLIPPY_SUCCESS" "$TEST_SUCCESS" "$STYLELINT_SUCCESS" "$ESLINT_SUCCESS")
NAME=(Zola Clippy Test Stylelint Eslint)

echo
for i in "${!NAME[@]}"; do
  if [ "${SUCCESS[$i]}" -eq 0 ]; then
    echo "${NAME[$i]} OK"
  else
    echo "${NAME[$i]} FAILED"
  fi
done
