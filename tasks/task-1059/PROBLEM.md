# task-1059 — Keep the same receiver contract

Return the stored reading through a shared reference and allow repeated calls. Implement the Sensor trait as declared.

Run `cargo check --manifest-path tasks/task-1059/Cargo.toml` from the course root. Locate E0053 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1059/Cargo.toml`.

For example, `Dial(33).read()` should produce `33`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0053](https://doc.rust-lang.org/1.88.0/error_codes/E0053.html)
