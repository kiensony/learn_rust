# task-2489 — Trailing raw reads · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced pointer-contract lab. Observe the final four bytes, or the whole input if shorter. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn trailing_raw_reads_longest_run(input: &[u8]) -> u64
```

## Public examples

- `trailing_raw_reads_longest_run(&[])` gives `0`.
- `trailing_raw_reads_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2489/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2489`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
