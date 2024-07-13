#!/usr/bin/env just --justfile

vault_key := env_var_or_default('VAULTKEY', "N/A")
profile := "dev"
store := "store-"
target := "x86_64-unknown-linux-gnu"

default: lint test

clean: 
  cargo clean

lint:
  cargo fmt --all -- --emit stdout
  cargo clippy --all-features -- -Dwarnings

test:
  cargo test --workspace

openapi:
  docker run --rm \
    -v ${PWD}/mgwconf-network:/mgwconf-network openapitools/openapi-generator-cli generate \
    -i /mgwconf-network/oas/SWIFT-API-mgw-configuration-api-2.0.0-swagger.json \
    -g rust \
    -o /mgwconf-network/api/configuration \
    -c /mgwconf-network/codegen.yml \
    --skip-validate-spec \
    --additional-properties=enumNameSuffix="MGWCONF",supportMultipleResponses=true,packageName="mgw-configuration"

  chown -R ${USER} ${PWD}/mgwconf-network/api

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
  cargo run --bin="mgwc_ui" --no-default-features --features="{{store}}ui" --profile={{profile}} --target={{target}} -- -k {{vault_key}} --ca=CA.pem {{ARGS}}