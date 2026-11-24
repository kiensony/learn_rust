# task-267 — Handle an empty name

Return zero for an empty input and the UTF-8 byte length for every other input. Keep the early return.

Run `cargo check --manifest-path tasks/task-267/Cargo.toml` from the course root. Locate E0069 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-267/Cargo.toml`.

For example, `badge_bytes("")` should produce `0`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0069](https://doc.rust-lang.org/1.88.0/error_codes/E0069.html)
