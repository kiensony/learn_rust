# task-516 — Count a separator

Count how many times a vertical bar occurs in the supplied UTF-8 string. Use str::matches.

Run `cargo check --manifest-path tasks/task-516/Cargo.toml` from the course root. Locate E0765 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-516/Cargo.toml`.

For example, `bars("a|b")` should produce `1`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0765](https://doc.rust-lang.org/1.88.0/error_codes/E0765.html)
