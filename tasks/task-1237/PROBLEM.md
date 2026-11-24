# task-1237 — Seek offsets · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Return distinct observations in ascending order. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `seek_offsets_distinct(&[])` gives `vec![]`.
- `seek_offsets_distinct(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1237/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1237`.

[Module reading](https://doc.rust-lang.org/std/io/)
