# task-756 — Collect a delivery queue

Collect a list of stop numbers in iteration order and append a final zero stop. Return an owned Vec<u16>.

Run `cargo check --manifest-path tasks/task-756/Cargo.toml` from the course root. Locate E0282 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-756/Cargo.toml`.

For example, `stops(&[5,8])` should produce `vec![5,8,0]`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0282](https://doc.rust-lang.org/1.88.0/error_codes/E0282.html)
