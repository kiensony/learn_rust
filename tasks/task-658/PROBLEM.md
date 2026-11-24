# task-658 — Describe every traffic light

Return "wait" for Red, "ready" for Amber and "go" for Green. Match every variant explicitly.

Run `cargo check --manifest-path tasks/task-658/Cargo.toml` from the course root. Locate E0004 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-658/Cargo.toml`.

For example, `instruction(Light::Green)` should produce `"go"`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0004](https://doc.rust-lang.org/1.88.0/error_codes/E0004.html)
