# task-1618 — Saturation property · steps

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: adding one with saturation leaves the value unchanged. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return the absolute differences between each adjacent pair, in order. Use an exhaustive u8 domain test with a model separate from the implementation. During review, point to the operation that enforces its contract.

```rust
pub fn saturation_property_steps(input: &[u8]) -> Vec<u32>
```

## Public examples

- `saturation_property_steps(&[])` gives `vec![]`.
- `saturation_property_steps(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1618/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1618`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
