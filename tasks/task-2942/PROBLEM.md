# task-2942 — Rollback triggers · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if new latency exceeds double the old, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return the sum of all observations, widening each one before adding. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn rollback_triggers_total(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `rollback_triggers_total(&[])` gives `0`.
- `rollback_triggers_total(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2942/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2942`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
