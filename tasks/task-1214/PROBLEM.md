# task-1214 — Read chunks · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Keep the first observation and every later one strictly above all earlier observations. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `read_chunks_peaks(&[])` gives `vec![]`.
- `read_chunks_peaks(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1214/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1214`.

[Module reading](https://doc.rust-lang.org/std/io/)
