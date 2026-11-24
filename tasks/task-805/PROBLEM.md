# task-805 — Convert a parsing error

Parse a decimal u16 with str::parse, propagate failure using ?, and return BadCount on invalid or out-of-range text.

Run `cargo check --manifest-path tasks/task-805/Cargo.toml` from the course root. Locate E0277 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-805/Cargo.toml`.

For example, `parse_count("19")` should produce `Ok(19)`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0277](https://doc.rust-lang.org/1.88.0/error_codes/E0277.html)
