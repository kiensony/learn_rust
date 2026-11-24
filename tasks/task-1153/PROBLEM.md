# task-1153 — Borrow a prefix from the caller

Return the part of an input string before its first colon, or the whole input if there is none. Borrow directly from the caller.

Run `cargo check --manifest-path tasks/task-1153/Cargo.toml` from the course root. Locate E0515 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1153/Cargo.toml`.

For example, `prefix("key:field")` should produce `"key"`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0515](https://doc.rust-lang.org/1.88.0/error_codes/E0515.html)
