# task-366 — Read the first temperature

Return the first i16 from a slice, or zero if it is empty. Keep the copied() call.

Run `cargo check --manifest-path tasks/task-366/Cargo.toml` from the course root. Locate E0614 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-366/Cargo.toml`.

For example, `first_reading(&[])` should produce `0`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0614](https://doc.rust-lang.org/1.88.0/error_codes/E0614.html)
