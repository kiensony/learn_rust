# task-1530 — Packet parity · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe one for odd bit parity and zero for even bit parity from each byte. During review locate the package, import and filesystem names in the manifest.

Return the observations in reverse order. Retain duplicate observations. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_parity_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packet_parity_reverse(&[])` gives `vec![]`.
- `packet_parity_reverse(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1530/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1530`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
