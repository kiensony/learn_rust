# task-1532 — Packet Gray code · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The package named track-wire is available under the local dependency alias wire. Use its public decode function. Observe the reflected binary Gray code x XOR (x shifted right by one) from each byte. During review locate the package, import and filesystem names in the manifest.

Return the sum of all observations, widening each one before adding. Use the bundled path dependency without duplicating its decoder. During review, point to the operation that enforces its contract.

```rust
pub fn packet_gray_code_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `packet_gray_code_total(&[])` gives `0`.
- `packet_gray_code_total(&[4])` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1532/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1532`.

[Module reading](https://doc.rust-lang.org/cargo/reference/workspaces.html)
