# task-2495 — Interior raw reads · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Advanced pointer-contract lab. Observe all bytes except the first and last; an input shorter than three yields no bytes. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn interior_raw_reads_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `interior_raw_reads_changes(&[])` gives `vec![]`.
- `interior_raw_reads_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2495/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2495`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
