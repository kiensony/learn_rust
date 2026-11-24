# task-1739 — Population macro · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Write a local macro_rules! macro accepting an expression and evaluating it exactly once per input byte. For each byte observe the number of one bits. In review, demonstrate the single-evaluation requirement with an expression that increments a counter.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a local declarative macro with an expr matcher. During review, point to the operation that enforces its contract.

```rust
pub fn population_macro_longest_run(input: &[u8]) -> u64
```

## Public examples

- `population_macro_longest_run(&[])` gives `0`.
- `population_macro_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1739/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1739`.

[Module reading](https://doc.rust-lang.org/reference/macros-by-example.html)
