# Example of Rust calling Lean code

A small example of calling Lean's FFI from Rust. Inspired by
[`aniva/RustCallLean`](https://git.leni.sh/aniva/RustCallLean), but
without Nix and using `lean-sys` rather than `bindgen` directly.

Resources:
- [Lean Reverse FFI](https://github.com/leanprover/lean4/tree/master/src/lake/examples/reverse-ffi)
- [`lean-sys` crate](https://github.com/digama0/lean-sys)
- [`lean-rs` crate](https://github.com/digama0/lean-rs)

