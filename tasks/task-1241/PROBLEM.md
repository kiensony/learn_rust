# task-1241 — Limited reads · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A bounded preview may consume at most four bytes. Observe every consumed byte including zero.

Return the observations in input order, including duplicates. Use Read::take and Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn limited_reads_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `limited_reads_trace(&[])` gives `vec![]`.
- `limited_reads_trace(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1241/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1241`.

[Module reading](https://doc.rust-lang.org/std/io/)
