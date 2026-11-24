# task-113 — Read an indicator byte

A device uses zero for idle and any nonzero byte for active. Return the corresponding Boolean.

Run `cargo check --manifest-path tasks/task-113/Cargo.toml` from the course root. Locate E0054 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-113/Cargo.toml`.

For example, `active(1)` should produce `true`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0054](https://doc.rust-lang.org/1.88.0/error_codes/E0054.html)
