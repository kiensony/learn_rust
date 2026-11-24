# task-2884 — Packed pin state · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the first byte rotated left by the second byte modulo eight per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Keep the first observation and every later one strictly above all earlier observations. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn packed_pin_state_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packed_pin_state_peaks(&[])` gives `vec![]`.
- `packed_pin_state_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2884/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2884`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
