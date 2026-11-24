# task-663 — Measure a packed shipment

A Box carries (item weight, packaging weight), while Letter carries a single weight. Sum the two components in u16 for Box.

Run `cargo check --manifest-path tasks/task-663/Cargo.toml` from the course root. Locate E0023 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-663/Cargo.toml`.

For example, `weight(Shipment::Box(3,4))` should produce `7`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0023](https://doc.rust-lang.org/1.88.0/error_codes/E0023.html)
