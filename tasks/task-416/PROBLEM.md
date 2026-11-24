# task-416 — Move a cursor forward

Advance the local cursor by one with saturation and return its new value. Keep the helper that borrows it mutably.

Run `cargo check --manifest-path tasks/task-416/Cargo.toml` from the course root. Locate E0596 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-416/Cargo.toml`.

For example, `next_cursor(5)` should produce `6`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0596](https://doc.rust-lang.org/1.88.0/error_codes/E0596.html)
