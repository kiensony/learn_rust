# task-1625 — Round trip property · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: doubling with wrapping then halving differs from the original. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn round_trip_property_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `round_trip_property_changes(&[])` gives `vec![]`.
- `round_trip_property_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1625/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1625`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
