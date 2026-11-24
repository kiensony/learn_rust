# task-510 — Recognize a tab character

Return true exactly when a character is the horizontal tab. Use a character literal.

Run `cargo check --manifest-path tasks/task-510/Cargo.toml` from the course root. Locate E0762 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-510/Cargo.toml`.

For example, `is_tab('\t')` should produce `true`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0762](https://doc.rust-lang.org/1.88.0/error_codes/E0762.html)
