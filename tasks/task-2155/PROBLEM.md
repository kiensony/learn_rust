# task-2155 — Linear search probes · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the count of equality probes required by a forward linear search for each input key, including the successful probe. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn linear_search_probes_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `linear_search_probes_changes(&[])` gives `vec![]`.
- `linear_search_probes_changes(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2155/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2155`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
