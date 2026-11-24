# task-218 — Return the last counter value

Return the greater of zero and the supplied counter, as a u16. Use a loop that produces its result via break.

Run `cargo check --manifest-path tasks/task-218/Cargo.toml` from the course root. Locate E0571 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-218/Cargo.toml`.

For example, `stop(3)` should produce `3`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0571](https://doc.rust-lang.org/1.88.0/error_codes/E0571.html)
