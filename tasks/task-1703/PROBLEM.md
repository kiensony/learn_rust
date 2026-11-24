# task-1703 — Pass a receiver into a macro

Expand the macro in Meter::value to read the supplied receiver’s field. Keep the method and invoke a macro from its body.

Run `cargo check --manifest-path tasks/task-1703/Cargo.toml` from the course root. Locate E0424 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1703/Cargo.toml`.

For example, `Meter(10).value()` should produce `10`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0424](https://doc.rust-lang.org/1.88.0/error_codes/E0424.html)
