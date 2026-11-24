# task-1261 — Positional options · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte lengths of arguments whose first character is not a hyphen. Do not spawn the child.

Return the observations in input order, including duplicates. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn positional_options_trace(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `positional_options_trace("")` gives `vec![]`.
- `positional_options_trace("only")` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1261/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1261`.

[Module reading](https://doc.rust-lang.org/std/process/)
