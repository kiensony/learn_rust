# task-1103 — Call through a trait object

Compute the current byte charge for any dynamically dispatched Meter implementation. Keep the borrowed trait-object parameter.

Run `cargo check --manifest-path tasks/task-1103/Cargo.toml` from the course root. Locate E0782 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1103/Cargo.toml`.

For example, `cost(&6u8)` should produce `6`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0782](https://doc.rust-lang.org/1.88.0/error_codes/E0782.html)
