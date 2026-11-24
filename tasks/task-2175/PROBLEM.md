# task-2175 — Prefix comparison budget · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Instrument an algorithm on a stream of keys. Observe the exact count of comparisons made when each new entry is compared to every earlier entry. State the worst-case time and auxiliary space in terms of stream length, and identify an input that realizes the bound.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use deterministic operation counts independent of wall-clock time. During review, point to the operation that enforces its contract.

```rust
pub fn prefix_comparison_budget_changes(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `prefix_comparison_budget_changes(&[])` gives `vec![]`.
- `prefix_comparison_budget_changes(&[4])` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2175/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2175`.

[Module reading](https://doc.rust-lang.org/std/primitive.slice.html)
