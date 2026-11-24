# task-1244 — Limited reads · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A bounded preview may consume at most four bytes. Observe every consumed byte including zero.

Keep the first observation and every later one strictly above all earlier observations. Use Read::take and Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn limited_reads_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `limited_reads_peaks(&[])` gives `vec![]`.
- `limited_reads_peaks(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1244/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1244`.

[Module reading](https://doc.rust-lang.org/std/io/)
