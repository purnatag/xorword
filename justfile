alias b := build

build:
    cargo build
    cargo run

test-all: build