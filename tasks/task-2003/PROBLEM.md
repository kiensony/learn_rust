# task-2003 — Await while reading a future

Return the value supplied by a future. Preserve suspension by awaiting it in a suitable function.

Run `cargo check --manifest-path tasks/task-2003/Cargo.toml` from the course root. Locate E0728 and identify the expression or declaration that triggered it. Open `src/lib.rs` in this exercise, make a targeted repair, and run `cargo test --manifest-path tasks/task-2003/Cargo.toml`.

For example, `{let x=reading(std::future::ready(7)); std::pin::pin!(x).poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))}` should produce `std::task::Poll::Ready(7)`. Keep the public API described above. Use safe Rust.

[Compiler explanation for E0728](https://doc.rust-lang.org/1.88.0/error_codes/E0728.html)
