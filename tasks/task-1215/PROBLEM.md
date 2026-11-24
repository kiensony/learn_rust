# task-1215 — Read chunks · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `read_chunks_changes(&[])` gives `vec![]`.
- `read_chunks_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1215/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1215`.

[Module reading](https://doc.rust-lang.org/std/io/)
