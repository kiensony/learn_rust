# task-2476 — Strided raw reads · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Advanced pointer-contract lab. Observe bytes at even zero-based offsets. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Count adjacent pairs for which the second observation is strictly smaller. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn strided_raw_reads_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `strided_raw_reads_descending_steps(&[])` gives `0`.
- `strided_raw_reads_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2476/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2476`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
