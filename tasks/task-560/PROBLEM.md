# task-560 — Choose a fallback title

Borrow the primary title when it is nonempty. Otherwise return the borrowed fallback. Preserve pointer identity and avoid allocation.

Run `cargo check --manifest-path tasks/task-560/Cargo.toml` from the course root. Locate E0106 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-560/Cargo.toml`.

For example, `title("map","fallback")` should produce `"map"`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0106](https://doc.rust-lang.org/1.88.0/error_codes/E0106.html)
