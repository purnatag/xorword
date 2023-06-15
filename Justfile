alias b := build

build:
    cargo build
    cargo run

run:
    cargo run --release

test-all: build
