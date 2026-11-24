# task-2403 — Give frame tags unique values

Use a byte-sized wire tag for each variant. Encode Start as 0, Data as 1 and End as 255. Retain explicit discriminants.

Run `cargo check --manifest-path tasks/task-2403/Cargo.toml` from the course root. Locate E0081 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-2403/Cargo.toml`.

For example, `encode(Tag::Start)` should produce `0`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0081](https://doc.rust-lang.org/1.88.0/error_codes/E0081.html)
