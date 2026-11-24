# task-2184 — Early exit probes · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Instrument an algorithm on a stream of keys. Observe the number of earlier entries inspected left-to-right, stopping at the first that is strictly greater than the current key, or after all earlier entries. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Keep the first observation and every later one strictly above all earlier observations. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn early_exit_probes_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `early_exit_probes_peaks(&[])` gives `vec![]`.
- `early_exit_probes_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2184/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2184`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
