# task-955 — Convert rows to an iterator

Return the sum of the supplied byte readings after doubling each in u64. Use map and sum.

Run `cargo check --manifest-path tasks/task-955/Cargo.toml` from the course root. Locate E0599 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-955/Cargo.toml`.

For example, `twice_all(&[1,7])` should produce `16`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0599](https://doc.rust-lang.org/1.88.0/error_codes/E0599.html)
