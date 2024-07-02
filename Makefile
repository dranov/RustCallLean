.PHONY: build clean

build: lib/RFFI.lean
	@echo "Building..."
	cd lib && lake build
	cargo build

clean:
	@echo "Cleaning..."
	cd lib && lake clean
	cargo clean