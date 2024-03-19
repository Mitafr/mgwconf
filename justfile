#!/usr/bin/env just --justfile

vault_key := env_var('VAULTKEY')
profile := "dev"
store := "store-"

default: lint build test

lint:
  cargo fmt --all
  cargo clippy --all-features -- -Dwarnings

test:
  cargo test --workspace

build:
  cargo build --bin="mgwc" --no-default-features --features="{{store}}cli"
  cargo build --bin="mgwc_ui" --no-default-features --features="{{store}}ui"

command $command:
  cargo run --bin="mgwc" --no-default-features --features="{{store}}cli" --profile={{profile}} -- -k {{vault_key}} -c $command

playbook $playbook:
  cargo run --bin="mgwc" --no-default-features --features="{{store}}cli" --profile={{profile}} -- -k {{vault_key}} -p $playbook

cli *ARGS:
  cargo run --bin="mgwc" --no-default-features --features="{{store}}cli" --profile={{profile}} -- -k {{vault_key}} {{ARGS}}

ui *ARGS:
  cargo run --bin="mgwc_ui" --no-default-features --features="{{store}}ui" --profile={{profile}} -- -k {{vault_key}} {{ARGS}}