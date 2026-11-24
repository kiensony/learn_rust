# task-609 — Record both sensor axes

Construct a public Point from east and north in that order. Store each reading once.

Run `cargo check --manifest-path tasks/task-609/Cargo.toml` from the course root. Locate E0062 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-609/Cargo.toml`.

For example, `locate(3,4)` should produce `Point {x:3,y:4}`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0062](https://doc.rust-lang.org/1.88.0/error_codes/E0062.html)
