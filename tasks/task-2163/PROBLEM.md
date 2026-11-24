# task-2163 — Sorted insertion moves · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the number of older elements shifted right when each key is inserted stably after any equals in a sorted vector. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return Some((minimum, maximum)) or None for an empty trace. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn sorted_insertion_moves_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `sorted_insertion_moves_bounds(&[])` gives `None`.
- `sorted_insertion_moves_bounds(&[4])` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2163/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2163`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
