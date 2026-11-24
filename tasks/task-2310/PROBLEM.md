# task-2310 — Warmup discard · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The input is a fixed sequence of benchmark durations in microseconds. Observe the measured durations after discarding the first two warmup iterations. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Return the observations in reverse order. Retain duplicate observations. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn warmup_discard_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `warmup_discard_reverse(&[])` gives `vec![]`.
- `warmup_discard_reverse(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2310/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2310`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
