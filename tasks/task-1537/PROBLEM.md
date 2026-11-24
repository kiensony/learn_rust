# task-1537 — Packet Gray code · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reflected binary Gray code x XOR (x shifted right by one) from each byte. During review locate the package, import and filesystem names in the manifest.

Return distinct observations in ascending order. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_gray_code_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `packet_gray_code_distinct(&[])` gives `vec![]`.
- `packet_gray_code_distinct(&[4])` gives `vec![6u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1537/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1537`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
