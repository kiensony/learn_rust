# task-1539 — Packet Gray code · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reflected binary Gray code x XOR (x shifted right by one) from each byte. During review locate the package, import and filesystem names in the manifest.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_gray_code_longest_run(input: &[u8]) -> u64
```

## Public examples

- `packet_gray_code_longest_run(&[])` gives `0`.
- `packet_gray_code_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1539/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1539`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
