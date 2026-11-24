# task-2874 — Flag register · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the bits enabled in both the flags byte and mask byte per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Keep the first observation and every later one strictly above all earlier observations. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn flag_register_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `flag_register_peaks(&[])` gives `vec![]`.
- `flag_register_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2874/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2874`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
