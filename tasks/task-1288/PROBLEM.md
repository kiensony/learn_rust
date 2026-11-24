# task-1288 — Option terminator · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte length of every argument following the first standalone -- token. Do not spawn the child.

Return the absolute differences between each adjacent pair, in order. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn option_terminator_steps(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `option_terminator_steps("")` gives `vec![]`.
- `option_terminator_steps("only")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1288/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1288`.

[Module reading](https://doc.rust-lang.org/std/process/)
