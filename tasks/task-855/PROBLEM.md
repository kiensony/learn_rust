# task-855 — Expose a conversion

Allow callers of this crate to invoke scale::quarter(value). Return integer division by four. Keep the caller using the module path.

Run `cargo check --manifest-path tasks/task-855/Cargo.toml` from the course root. Locate E0603 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-855/Cargo.toml`.

For example, `scale::quarter(8)` should produce `2`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0603](https://doc.rust-lang.org/1.88.0/error_codes/E0603.html)
