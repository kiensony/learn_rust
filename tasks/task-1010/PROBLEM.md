# task-1010 — Choose the pair type

Return the first and second u16 unchanged in an Ordered pair. Keep Ordered generic over both field types.

Run `cargo check --manifest-path tasks/task-1010/Cargo.toml` from the course root. Locate E0107 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1010/Cargo.toml`.

For example, `pair(4,7)` should produce `Ordered(4,7)`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0107](https://doc.rust-lang.org/1.88.0/error_codes/E0107.html)
