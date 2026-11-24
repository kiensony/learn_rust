# task-311 — Send a label with its size

Consume the String, then return the original allocation together with its UTF-8 byte length. Keep the incoming buffer.

Run `cargo check --manifest-path tasks/task-311/Cargo.toml` from the course root. Locate E0382 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-311/Cargo.toml`.

For example, `tag("cat".into())` should produce `("cat".into(), 3)`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0382](https://doc.rust-lang.org/1.88.0/error_codes/E0382.html)
