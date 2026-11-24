# task-707 — Return an optional reading

Return the number inside Some and zero for None. Handle both inputs explicitly.

Run `cargo check --manifest-path tasks/task-707/Cargo.toml` from the course root. Locate E0005 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-707/Cargo.toml`.

For example, `reading(Some(-4))` should produce `-4`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0005](https://doc.rust-lang.org/1.88.0/error_codes/E0005.html)
