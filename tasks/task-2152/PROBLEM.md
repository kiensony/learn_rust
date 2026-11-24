# task-2152 — Linear search probes · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the count of equality probes required by a forward linear search for each input key, including the successful probe. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the sum of all observations, widening each one before adding. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn linear_search_probes_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `linear_search_probes_total(&[])` gives `0`.
- `linear_search_probes_total(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2152/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2152`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
