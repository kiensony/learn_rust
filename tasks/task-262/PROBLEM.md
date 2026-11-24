# task-262 — Convert kitchen scoops

One scoop holds 12 grams. Return the mass for the supplied count by calling the provided scale helper.

Run `cargo check --manifest-path tasks/task-262/Cargo.toml` from the course root. Locate E0061 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-262/Cargo.toml`.

For example, `mass(0)` should produce `0`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0061](https://doc.rust-lang.org/1.88.0/error_codes/E0061.html)
