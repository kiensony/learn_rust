# task-1239 — Seek offsets · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_longest_run(input: &[u8]) -> u64
```

## Public examples

- `seek_offsets_longest_run(&[])` gives `0`.
- `seek_offsets_longest_run(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1239/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1239`.

[Module reading](https://doc.rust-lang.org/std/io/)
