# task-2877 — One sensor decoder for a board and browser

Embedded and WebAssembly lab. A packed input uses the lower 12 bits for an unsigned sample and bit 15 as valid. Return None when bit 15 is clear. Otherwise return Some with the sample. The adapters in device and web call this exact function. On Linux install the pinned targets with rustup target add --toolchain 1.88.0 thumbv7em-none-eabi wasm32-unknown-unknown. From this lab run cargo check --manifest-path device/Cargo.toml --target thumbv7em-none-eabi and cargo build --release --manifest-path web/Cargo.toml --target wasm32-unknown-unknown. Host tests cover validity and masking.

```rust
pub fn unpack(raw: u16) -> Option<u16>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `unpack(0)` gives `None`.
- `unpack(0x8005)` gives `Some(5)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2877/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2877`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
