# task-1054 — Implement the capacity contract

Provide the missing Capacity method for Shelf so a caller can read the exact stored number of slots.

Run `cargo check --manifest-path tasks/task-1054/Cargo.toml` from the course root. Locate E0046 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1054/Cargo.toml`.

For example, `Shelf(9).slots()` should produce `9`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0046](https://doc.rust-lang.org/1.88.0/error_codes/E0046.html)
