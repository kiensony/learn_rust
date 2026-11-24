# task-2196 — Linear search probes · threshold crossing

Instrument an algorithm on a stream of keys. Observe the count of equality probes required by a forward linear search for each input key, including the successful probe. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use deterministic operation counts independent of wall-clock time. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn linear_search_probes_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `linear_search_probes_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 1))`.
- `linear_search_probes_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2196/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2196`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
