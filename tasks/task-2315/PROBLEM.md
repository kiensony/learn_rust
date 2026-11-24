# task-2315 — Paired speedup · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe nonnegative improvements in complete before/after pairs, old minus new with zero as the floor. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn paired_speedup_changes(input: &[u8]) -> Vec<u32>
```

## Public examples

- `paired_speedup_changes(&[])` gives `vec![]`.
- `paired_speedup_changes(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2315/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2315`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
