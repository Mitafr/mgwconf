#!/usr/bin/env just --justfile

vault_key := env_var('VAULTKEY')
profile := "dev"
store := "store-"
target := "x86_64-unknown-linux-gnu"

default: lint build test

clean: 
  cargo clean

lint:
  cargo fmt --all
  cargo clippy --all-features -- -Dwarnings

test:
  cargo test --workspace

build:
  cargo build --bin="mgwc" --no-default-features --target={{target}} --features="{{store}}cli"
  cargo build --bin="mgwc_ui" --no-default-features --target={{target}} --features="{{store}}ui"

release:
  cargo build --bin="mgwc" --no-default-features --release --target={{target}} --features="{{store}}cli"
  cargo build --bin="mgwc_ui" --no-default-features --release --target={{target}} --features="{{store}}ui"

command $command:
  cargo run --bin="mgwc" --no-default-features --features="{{store}}cli" --profile={{profile}} --target={{target}} -- -k {{vault_key}} -c $command

playbook $playbook:
  cargo run --bin="mgwc" --no-default-features --features="{{store}}cli" --profile={{profile}} --target={{target}} -- -k {{vault_key}} -p $playbook

cli *ARGS:
  cargo run --bin="mgwc" --no-default-features --features="{{store}}cli" --profile={{profile}} --target={{target}} -- -k {{vault_key}} {{ARGS}}

ui *ARGS:
  cargo run --bin="mgwc_ui" --no-default-features --features="{{store}}ui" --profile={{profile}} --target={{target}} -- -k {{vault_key}} {{ARGS}}