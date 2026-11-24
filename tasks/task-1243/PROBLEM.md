# task-1243 — Limited reads · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A bounded preview may consume at most four bytes. Observe every consumed byte including zero.

Return Some((minimum, maximum)) or None for an empty trace. Use Read::take and Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn limited_reads_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `limited_reads_bounds(&[])` gives `None`.
- `limited_reads_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1243/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1243`.

[Module reading](https://doc.rust-lang.org/std/io/)
