# task-1803 — Keep inputs alive in a worker

Spawn a thread that returns its input plus one with saturation, and join it before returning. Keep thread::spawn.

Run `cargo check --manifest-path tasks/task-1803/Cargo.toml` from the course root. Locate E0373 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1803/Cargo.toml`.

For example, `worker(5)` should produce `6`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0373](https://doc.rust-lang.org/1.88.0/error_codes/E0373.html)
