# task-1232 — Seek offsets · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Return the sum of all observations, widening each one before adding. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `seek_offsets_total(&[])` gives `0`.
- `seek_offsets_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1232/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1232`.

[Module reading](https://doc.rust-lang.org/std/io/)
