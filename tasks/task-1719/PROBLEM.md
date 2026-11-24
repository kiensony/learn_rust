# task-1719 — Saturating macro · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the byte plus four with u8 saturation. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn saturating_macro_longest_run(input: &[u8]) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `saturating_macro_longest_run(&[])` gives `0`.
- `saturating_macro_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1719/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1719`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
