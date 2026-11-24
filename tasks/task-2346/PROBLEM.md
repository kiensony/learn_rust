# task-2346 — Warmup discard · threshold crossing

The input is a fixed sequence of benchmark durations in microseconds. Observe the measured durations after discarding the first two warmup iterations. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use deterministic measurement fixtures and a release build. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn warmup_discard_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `warmup_discard_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 9))`.
- `warmup_discard_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2346/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2346`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
