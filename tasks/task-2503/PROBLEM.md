# task-2503 — Respect the callback ABI

Call a C-ABI callback with a u16 and return its result. Publish a C-ABI callback type alias so other crates can pass compatible functions.

Run `cargo check --manifest-path tasks/task-2503/Cargo.toml` from the course root. Locate E0308 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-2503/Cargo.toml`.

For example, `invoke(default_callback(),7)` should produce `7`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0308](https://doc.rust-lang.org/1.88.0/error_codes/E0308.html)
