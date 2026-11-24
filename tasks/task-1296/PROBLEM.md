# task-1296 — Command arguments · threshold crossing

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect each argument's byte length. Do not spawn the child.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use Command::args and Command::get_args. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn command_arguments_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `command_arguments_threshold_crossing("input -v -q input --level=255", 0)` gives `Some((0, 5))`.
- `command_arguments_threshold_crossing("", 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1296/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1296`.

[Module reading](https://doc.rust-lang.org/std/process/)
