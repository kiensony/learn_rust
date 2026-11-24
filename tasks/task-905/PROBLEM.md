# task-905 — Apply a discount closure

Invoke the supplied two-argument discount function on the list price and the rebate in that order.

Run `cargo check --manifest-path tasks/task-905/Cargo.toml` from the course root. Locate E0057 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-905/Cargo.toml`.

For example, `quote(|a,b|a-b,12,4)` should produce `8`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0057](https://doc.rust-lang.org/1.88.0/error_codes/E0057.html)
