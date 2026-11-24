# task-2865 — Big endian registers · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Decode a portable register image for a host and WebAssembly consumer. Each record occupies two bytes. Ignore a trailing unpaired byte. Observe the big-endian unsigned 16-bit reading per complete record. Verify with cargo check --target wasm32-unknown-unknown after installing the target, and explain the byte order contract.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use fixed-width integer operations independent of host byte order. During review, point to the operation that enforces its contract.

```rust
pub fn big_endian_registers_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `big_endian_registers_changes(&[])` gives `vec![]`.
- `big_endian_registers_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2865/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2865`.

[Module reading](https://doc.rust-lang.org/rustc/platform-support.html)
