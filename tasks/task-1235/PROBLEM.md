# task-1235 — Seek offsets · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `seek_offsets_changes(&[])` gives `vec![]`.
- `seek_offsets_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1235/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1235`.

[Module reading](https://doc.rust-lang.org/std/io/)
