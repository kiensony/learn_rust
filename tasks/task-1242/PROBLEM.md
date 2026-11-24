# task-1242 — Limited reads · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A bounded preview may consume at most four bytes. Observe every consumed byte including zero.

Return the sum of all observations, widening each one before adding. Use Read::take and Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn limited_reads_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `limited_reads_total(&[])` gives `0`.
- `limited_reads_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1242/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1242`.

[Module reading](https://doc.rust-lang.org/std/io/)
