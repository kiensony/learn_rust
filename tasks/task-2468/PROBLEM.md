# task-2468 — Reverse raw reads · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced pointer-contract lab. Observe all bytes in reverse order. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return the absolute differences between each adjacent pair, in order. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn reverse_raw_reads_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `reverse_raw_reads_steps(&[])` gives `vec![]`.
- `reverse_raw_reads_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2468/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2468`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
