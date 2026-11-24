# task-1233 — Seek offsets · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Return Some((minimum, maximum)) or None for an empty trace. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `seek_offsets_bounds(&[])` gives `None`.
- `seek_offsets_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1233/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1233`.

[Module reading](https://doc.rust-lang.org/std/io/)
