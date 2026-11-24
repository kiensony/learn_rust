# task-2483 — Trailing raw reads · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced pointer-contract lab. Observe the final four bytes, or the whole input if shorter. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return Some((minimum, maximum)) or None for an empty trace. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn trailing_raw_reads_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `trailing_raw_reads_bounds(&[])` gives `None`.
- `trailing_raw_reads_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2483/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2483`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
