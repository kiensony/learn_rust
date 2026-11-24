# task-1299 — Option terminator · frequent readings

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte length of every argument following the first standalone -- token. Do not spawn the child.

Return up to keep distinct observations ordered by descending frequency, breaking ties by first appearance in the trace. Return an empty vector for keep zero. Each observation value appears at most once in the result.

Use Command::args and Command::get_args. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn option_terminator_frequent_readings(input: &str, keep: usize) -> Vec<u32>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `option_terminator_frequent_readings("-- --level=4 x", 3)` gives `vec![9u32, 1u32]`.
- `option_terminator_frequent_readings("", 0)` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1299/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1299`.

[Module reading](https://doc.rust-lang.org/std/process/)
