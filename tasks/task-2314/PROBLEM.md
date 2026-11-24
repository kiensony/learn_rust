# task-2314 — Paired speedup · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

The input is a fixed sequence of benchmark durations in microseconds. Observe nonnegative improvements in complete before/after pairs, old minus new with zero as the floor. Run the acceptance suite with both cargo test and cargo test --release, and explain how compiler optimization could affect a timing-based assertion.

Keep the first observation and every later one strictly above all earlier observations. Use deterministic measurement fixtures and a release build. During review, point to the operation that enforces its contract.

```rust
pub fn paired_speedup_peaks(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `paired_speedup_peaks(&[])` gives `vec![]`.
- `paired_speedup_peaks(&[4])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2314/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2314`.

[Module reading](https://doc.rust-lang.org/cargo/reference/profiles.html)
