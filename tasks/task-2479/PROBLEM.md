# task-2479 — Strided raw reads · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Advanced pointer-contract lab. Observe bytes at even zero-based offsets. For each visited offset construct the corresponding raw pointer with wrapping_add and check that it equals the address of the indexed byte. Keep access through the live slice. During review state which allocation owns each address and explain why a one-past pointer cannot be read.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use slice::as_ptr, pointer equality, and safe indexed access. During review, point to the operation that enforces its contract.

```rust
pub fn strided_raw_reads_longest_run(input: &[u8]) -> u64
```

## Public examples

- `strided_raw_reads_longest_run(&[])` gives `0`.
- `strided_raw_reads_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2479/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2479`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
