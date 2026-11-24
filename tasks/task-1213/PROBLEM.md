# task-1213 — Read chunks · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Return Some((minimum, maximum)) or None for an empty trace. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `read_chunks_bounds(&[])` gives `None`.
- `read_chunks_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1213/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1213`.

[Module reading](https://doc.rust-lang.org/std/io/)
