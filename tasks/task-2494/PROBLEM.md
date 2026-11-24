# task-2494 — Interior raw reads · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Advanced pointer-contract lab. Observe all bytes except the first and last; an input shorter than three yields no bytes. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Keep the first observation and every later one strictly above all earlier observations. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn interior_raw_reads_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `interior_raw_reads_peaks(&[])` gives `vec![]`.
- `interior_raw_reads_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2494/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2494`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
