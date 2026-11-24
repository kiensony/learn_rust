# task-2303 — Warmup discard · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

The input is a fixed sequence of benchmark durations in microseconds. Observe the measured durations after discarding the first two warmup iterations. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return Some((minimum, maximum)) or None for an empty trace. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn warmup_discard_bounds(input: &[u8]) -> Option<(u32, u32)>
```

## Public examples

- `warmup_discard_bounds(&[])` gives `None`.
- `warmup_discard_bounds(&[4])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2303/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2303`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
