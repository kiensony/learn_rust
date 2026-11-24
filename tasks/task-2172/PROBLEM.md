# task-2172 — Prefix comparison budget · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the exact count of comparisons made when each new entry is compared to every earlier entry. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the sum of all observations, widening each one before adding. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn prefix_comparison_budget_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `prefix_comparison_budget_total(&[])` gives `0`.
- `prefix_comparison_budget_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2172/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2172`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
