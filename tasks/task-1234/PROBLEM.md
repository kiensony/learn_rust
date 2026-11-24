# task-1234 — Seek offsets · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Observe the byte at each odd zero-based offset in a random-access file. A one-byte file has no observations.

Keep the first observation and every later one strictly above all earlier observations. Use Seek and Read::read_exact on a Cursor. During review, point to the operation that enforces its contract.

```rust
pub fn seek_offsets_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `seek_offsets_peaks(&[])` gives `vec![]`.
- `seek_offsets_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1234/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1234`.

[Module reading](https://doc.rust-lang.org/std/io/)
