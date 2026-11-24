# task-466 — Read a three-pixel strip

Return the red channel (first entry) from a three-byte RGB array. Keep array destructuring.

Run `cargo check --manifest-path tasks/task-466/Cargo.toml` from the course root. Locate E0527 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-466/Cargo.toml`.

For example, `red([9,2,1])` should produce `9`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0527](https://doc.rust-lang.org/1.88.0/error_codes/E0527.html)
