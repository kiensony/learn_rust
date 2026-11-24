# task-212 — Break from the right loop

Return the first even number or None. Keep the labeled loop that exits early.

Run `cargo check --manifest-path tasks/task-212/Cargo.toml` from the course root. Locate E0426 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-212/Cargo.toml`.

For example, `first_even(&[3, 4, 2])` should produce `Some(4)`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0426](https://doc.rust-lang.org/1.88.0/error_codes/E0426.html)
