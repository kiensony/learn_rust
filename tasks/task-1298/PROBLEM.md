# task-1298 — Explicit levels · rolling load

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect numeric values of all --level=N tokens whose N parses as u8; ignore other tokens. Do not spawn the child.

For each complete sliding window of width observations, return its sum as u64. Consecutive windows overlap by width minus one observations. Width zero and widths greater than the trace length yield an empty vector.

Use Command::args and Command::get_args. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn explicit_levels_rolling_load(input: &str, width: usize) -> Vec<u64>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `explicit_levels_rolling_load("--level=3 data --level=9", 1)` gives `vec![3u64, 9u64]`.
- `explicit_levels_rolling_load("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1298/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1298`.

[Module reading](https://doc.rust-lang.org/std/process/)
