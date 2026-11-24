# task-168 — Choose a station label

Return the string "offline" for zero stations and "online" for every positive count.

Run `cargo check --manifest-path tasks/task-168/Cargo.toml` from the course root. Locate E0308 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-168/Cargo.toml`.

For example, `status(0)` should produce `"offline"`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0308](https://doc.rust-lang.org/1.88.0/error_codes/E0308.html)
