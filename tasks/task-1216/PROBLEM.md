# task-1216 — Read chunks · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Count adjacent pairs for which the second observation is strictly smaller. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_descending_steps(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `read_chunks_descending_steps(&[])` gives `0`.
- `read_chunks_descending_steps(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1216/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1216`.

[Module reading](https://doc.rust-lang.org/std/io/)
