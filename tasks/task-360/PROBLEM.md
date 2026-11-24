# task-360 — Count letters without taking a name

Return the number of Unicode scalar values in the borrowed string. Preserve the original owner and avoid allocation.

Run `cargo check --manifest-path tasks/task-360/Cargo.toml` from the course root. Locate E0507 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-360/Cargo.toml`.

For example, `letter_count(&"ink".into())` should produce `3`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0507](https://doc.rust-lang.org/1.88.0/error_codes/E0507.html)
