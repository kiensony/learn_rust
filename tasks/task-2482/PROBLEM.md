# task-2482 — Trailing raw reads · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Advanced pointer-contract lab. Observe the final four bytes, or the whole input if shorter. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return the sum of all observations, widening each one before adding. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn trailing_raw_reads_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `trailing_raw_reads_total(&[])` gives `0`.
- `trailing_raw_reads_total(&[4])` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2482/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2482`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
