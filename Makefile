SHELL=/bin/bash
CARGO = cargo
WASMPACK = wasm-pack

# Build the project in debug mode
.PHONY: build
build:
	$(CARGO) build $(CARGO_FLAGS)

# Check the format of the source code
.PHONY: fmt-check
fmt:
	cargo fmt --all -- --check

# fmt fix
.PHONY: fmt
fmt:
	cargo fmt

# Run the tests
.PHONY: test
test:
	$(CARGO) test -- --nocapture

# Wasm build
.PHONY: wasm
wasm:
	$(WASMPACK) build --scope kazukiyoshida


n: 0
.PHONY: step
step:
	$(CARGO) run --example step ${n};

.PHONY: flow
flow:
	$(CARGO) run --example flow;

# Publis wasm package
.PHONY: npm-publish
npm-publish:
	cd pkg; npm publish --registry http://localhost:4873
