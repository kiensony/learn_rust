# task-2180 — Prefix comparison budget · reverse

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Instrument an algorithm on a stream of keys. Observe the exact count of comparisons made when each new entry is compared to every earlier entry. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the observations in reverse order. Retain duplicate observations. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn prefix_comparison_budget_reverse(input: &[u8]) -> Vec<u32>
```

## Public examples

- `prefix_comparison_budget_reverse(&[])` gives `vec![]`.
- `prefix_comparison_budget_reverse(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2180/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2180`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
