# task-1619 — Saturation property · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: adding one with saturation leaves the value unchanged. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn saturation_property_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturation_property_longest_run(&[])` gives `0`.
- `saturation_property_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1619/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1619`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
