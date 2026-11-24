# task-2929 — Failure threshold · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

An upgrade incident log records old and new latency ticks as consecutive bytes for each probe. Ignore a final incomplete pair. Observe one if the old latency was below 100 and the new one reaches or exceeds 100, else zero per complete pair. Provide the smallest probe that triggered the starter fault and a regression test that protects it.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use paired measurements, widening or saturation appropriate to the metric. During review, point to the operation that enforces its contract.

```rust
pub fn failure_threshold_longest_run(input: &[u8]) -> u64
```

## Public examples

- `failure_threshold_longest_run(&[])` gives `0`.
- `failure_threshold_longest_run(&[4])` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2929/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2929`.

[Module reading](https://doc.rust-lang.org/cargo/reference/resolver.html)
