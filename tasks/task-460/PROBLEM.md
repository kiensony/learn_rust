# task-460 — Read a tuple coordinate

Return a point’s vertical coordinate. Keep the pair as an (i16, i16) tuple.

Run `cargo check --manifest-path tasks/task-460/Cargo.toml` from the course root. Locate E0608 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-460/Cargo.toml`.

For example, `height((1,9))` should produce `9`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0608](https://doc.rust-lang.org/1.88.0/error_codes/E0608.html)
