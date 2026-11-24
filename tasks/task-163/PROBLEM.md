# task-163 — Price a printed page

Return 5 for a color page and 2 for a monochrome page. Return the value of the whole branching expression.

Run `cargo check --manifest-path tasks/task-163/Cargo.toml` from the course root. Locate E0317 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-163/Cargo.toml`.

For example, `price(true)` should produce `5`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0317](https://doc.rust-lang.org/1.88.0/error_codes/E0317.html)
