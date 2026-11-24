# task-1254 — Command arguments · peaks

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect each argument's byte length. Do not spawn the child.

Keep the first observation and every later one strictly above all earlier observations. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn command_arguments_peaks(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `command_arguments_peaks("")` gives `vec![]`.
- `command_arguments_peaks("only")` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1254/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1254`.

[Module reading](https://doc.rust-lang.org/std/process/)
