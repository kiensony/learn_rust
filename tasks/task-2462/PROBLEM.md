# task-2462 — Reverse raw reads · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced pointer-contract lab. Observe all bytes in reverse order. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return the sum of all observations, widening each one before adding. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn reverse_raw_reads_total(input: &[u8]) -> u64
```

## Public examples

- `reverse_raw_reads_total(&[])` gives `0`.
- `reverse_raw_reads_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2462/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2462`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
