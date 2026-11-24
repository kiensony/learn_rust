# task-1303 — Convert a parsed optional port

For an optional u8 port, return the decimal string length or zero when missing. Use Option::map with the supplied helper.

Run `cargo check --manifest-path tasks/task-1303/Cargo.toml` from the course root. Locate E0631 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-1303/Cargo.toml`.

For example, `port_width(Some(80))` should produce `2`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0631](https://doc.rust-lang.org/1.88.0/error_codes/E0631.html)
