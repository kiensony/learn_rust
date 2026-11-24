# task-2179 — Prefix comparison budget · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Instrument an algorithm on a stream of keys. Observe the exact count of comparisons made when each new entry is compared to every earlier entry. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn prefix_comparison_budget_longest_run(input: &[u8]) -> u64
```

## Public examples

- `prefix_comparison_budget_longest_run(&[])` gives `0`.
- `prefix_comparison_budget_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2179/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2179`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
