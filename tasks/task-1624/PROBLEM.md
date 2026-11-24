# task-1624 — Round trip property · peaks

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: doubling with wrapping then halving differs from the original. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Keep the first observation and every later one strictly above all earlier observations. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn round_trip_property_peaks(input: &[u8]) -> Vec<u32>
```

## Public examples

- `round_trip_property_peaks(&[])` gives `vec![]`.
- `round_trip_property_peaks(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1624/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1624`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
