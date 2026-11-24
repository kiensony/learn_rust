# task-2153 — Linear search probes · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the count of equality probes required by a forward linear search for each input key, including the successful probe. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return Some((minimum, maximum)) or None for an empty trace. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn linear_search_probes_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `linear_search_probes_bounds(&[])` gives `None`.
- `linear_search_probes_bounds(&[4])` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2153/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2153`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
