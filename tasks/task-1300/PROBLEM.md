# task-1300 — Quiet option positions · replay mismatch

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect zero-based positions of exact -q arguments, wherever they appear. Do not spawn the child.

Compare the derived observations with a stored replay. Return the first unequal position. If one trace ends early, return the length of the shorter trace. Return None only if both contents and length match.

Use Command::args and Command::get_args. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn quiet_option_positions_replay_mismatch(input: &str, replay: &[u32]) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `quiet_option_positions_replay_mismatch("-q -q -- -v", &[0])` gives `Some(1)`.
- `quiet_option_positions_replay_mismatch("", &[])` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1300/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1300`.

[Module reading](https://doc.rust-lang.org/std/process/)
