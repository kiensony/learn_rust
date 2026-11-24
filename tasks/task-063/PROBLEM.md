# task-063 — Keep a value in scope

Return the supplied temperature unchanged. Keep the local binding named reading.

Run `cargo check --manifest-path tasks/task-063/Cargo.toml` from the course root. Locate E0425 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-063/Cargo.toml`.

For example, `degrees(-4)` should produce `-4`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0425](https://doc.rust-lang.org/1.88.0/error_codes/E0425.html)
