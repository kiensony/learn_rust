# task-1238 — Seek offsets · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Return the absolute differences between each adjacent pair, in order. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_steps(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `seek_offsets_steps(&[])` gives `vec![]`.
- `seek_offsets_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1238/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1238`.

[Module reading](https://doc.rust-lang.org/std/io/)
