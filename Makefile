SHELL=/bin/bash
CARGO = cargo

# Build the project in debug mode
.PHONY: build
build:
	$(CARGO) build $(CARGO_FLAGS)

# Check the format of the source code
.PHONY: fmt
fmt:
	cargo fmt --all -- --check

# Run the tests
.PHONY: test
test:
	$(CARGO) test --all-features


n: 0
.PHONY: step
step:
	$(CARGO) run --example step ${n};

.PHONY: flow
flow:
	$(CARGO) run --example flow;
