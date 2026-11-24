# task-1212 — Read chunks · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Return the sum of all observations, widening each one before adding. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_total(input: &[u8]) -> u64
```

## Public examples

- `read_chunks_total(&[])` gives `0`.
- `read_chunks_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1212/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1212`.

[Module reading](https://doc.rust-lang.org/std/io/)
