# task-1629 — Round trip property · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: doubling with wrapping then halving differs from the original. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn round_trip_property_longest_run(input: &[u8]) -> u64
```

## Public examples

- `round_trip_property_longest_run(&[])` gives `0`.
- `round_trip_property_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1629/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1629`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
