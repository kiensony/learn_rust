# task-410 — Append a snapshot of the first item

If the vector is nonempty, append a copy of its first number, then return that number. For an empty vector return None and leave it empty.

Run `cargo check --manifest-path tasks/task-410/Cargo.toml` from the course root. Locate E0502 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-410/Cargo.toml`.

For example, `{let mut v = vec![8, 3]; (repeat_first(&mut v), v)}` should produce `(Some(8), vec![8,3,8])`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0502](https://doc.rust-lang.org/1.88.0/error_codes/E0502.html)
