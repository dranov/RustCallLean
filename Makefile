.PHONY: build run clean

build: lib/Callee.lean
	@echo "Building..."
	cargo build

run:
	cargo run

clean:
	@echo "Cleaning..."
	cd lib && lake clean
	cargo clean