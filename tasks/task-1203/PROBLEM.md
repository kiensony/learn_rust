# task-1203 — Update a buffered read count

Add newly read bytes to a caller-owned counter using saturation. The caller must observe the update.

Run `cargo check --manifest-path tasks/task-1203/Cargo.toml` from the course root. Locate E0594 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1203/Cargo.toml`.

For example, `{let mut n=4;record(&mut n,7);n}` should produce `11`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0594](https://doc.rust-lang.org/1.88.0/error_codes/E0594.html)
