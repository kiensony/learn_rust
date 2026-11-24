# task-1004 — Return a generic stack array

Return a zero-filled array of N bytes. Keep the result sized at compile time and support N equal to zero.

Run `cargo check --manifest-path tasks/task-1004/Cargo.toml` from the course root. Locate E0401 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1004/Cargo.toml`.

For example, `clear::<3>()` should produce `[0;3]`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0401](https://doc.rust-lang.org/1.88.0/error_codes/E0401.html)
