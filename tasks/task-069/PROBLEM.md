# task-069 — Reset a busy flag

Set the local flag to false when reset is true. Otherwise return its initial value. Keep the reassignment.

Run `cargo check --manifest-path tasks/task-069/Cargo.toml` from the course root. Locate E0384 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-069/Cargo.toml`.

For example, `busy(true, true)` should produce `false`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0384](https://doc.rust-lang.org/1.88.0/error_codes/E0384.html)
