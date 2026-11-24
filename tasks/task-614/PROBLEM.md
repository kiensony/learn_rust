# task-614 — Measure a parcel edge

Return the longer edge of the Parcel. Keep the public width and height fields.

Run `cargo check --manifest-path tasks/task-614/Cargo.toml` from the course root. Locate E0609 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-614/Cargo.toml`.

For example, `edge(Parcel{width:1,height:4})` should produce `4`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0609](https://doc.rust-lang.org/1.88.0/error_codes/E0609.html)
