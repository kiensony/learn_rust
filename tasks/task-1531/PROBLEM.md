# task-1531 — Packet Gray code · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reflected binary Gray code x XOR (x shifted right by one) from each byte. During review locate the package, import and filesystem names in the manifest.

Return the observations in input order, including duplicates. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_gray_code_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_gray_code_trace(&[])` gives `vec![]`.
- `packet_gray_code_trace(&[4])` gives `vec![6u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1531/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1531`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
