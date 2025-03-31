# rust-template

Rust template code

`rustc --version`

`cargo --version`

## Init Project

cargo new rusttemplate --vcs=git

## Build

cargo build

## Run

cargo run --bin solver

cargo run --bin rusttemplate

## Test

cargo test

## Containerize

docker build -t rust-api .

## Running GitHub actions locally

https://github.com/nektos/act

`act -W .github/workflows/ci.yaml -j lint-format-test`
