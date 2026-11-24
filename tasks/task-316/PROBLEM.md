# task-316 — Release a ticket explicitly

Release the supplied ticket before returning its original byte length. Keep explicit early release.

Run `cargo check --manifest-path tasks/task-316/Cargo.toml` from the course root. Locate E0040 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-316/Cargo.toml`.

For example, `release("ABC".into())` should produce `3`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0040](https://doc.rust-lang.org/1.88.0/error_codes/E0040.html)
