# task-1646 — Parity property · threshold crossing

Each byte is a generated test input. Observe one when the following predicate holds and zero otherwise: the remainder on division by two differs from the low bit. As a separate regression test, enumerate the full u8 domain; check the property against an independently computed model and report the smallest counterexample.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use an exhaustive u8 domain test with a model separate from the implementation. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn parity_property_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `parity_property_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 0))`.
- `parity_property_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1646/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1646`.

[Module reading](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
