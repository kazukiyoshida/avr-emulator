build:
  cargo build

test: build
  cargo test -- --nocapture

avr: build
  cargo test test_atmega328p -- --nocapture

# Builds the library.
core:
	cargo run --bin core
