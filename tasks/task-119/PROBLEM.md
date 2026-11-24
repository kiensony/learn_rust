# task-119 — Double within byte capacity

Add the argument to a base value of 200 as an eight-bit saturating sum.

Run `cargo check --manifest-path tasks/task-119/Cargo.toml` from the course root. Locate E0689 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-119/Cargo.toml`.

For example, `offset(2)` should produce `202`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0689](https://doc.rust-lang.org/1.88.0/error_codes/E0689.html)
