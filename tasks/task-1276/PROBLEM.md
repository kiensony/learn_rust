# task-1276 — Explicit levels · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect numeric values of all --level=N tokens whose N parses as u8; ignore other tokens. Do not spawn the child.

Count adjacent pairs for which the second observation is strictly smaller. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn explicit_levels_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `explicit_levels_descending_steps("")` gives `0`.
- `explicit_levels_descending_steps("only")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1276/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1276`.

[Module reading](https://doc.rust-lang.org/std/process/)
