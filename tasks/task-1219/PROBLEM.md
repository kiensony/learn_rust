# task-1219 — Read chunks · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Read a cursor in blocks of three bytes. Observe the actual number of bytes in every successful nonempty read. Keep a final short block.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Read::read and a fixed buffer. During review, point to the operation that enforces its contract.

```rust
pub fn read_chunks_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `read_chunks_longest_run(&[])` gives `0`.
- `read_chunks_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1219/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1219`.

[Module reading](https://doc.rust-lang.org/std/io/)
