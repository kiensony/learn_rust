# task-1282 — Option terminator · total

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte length of every argument following the first standalone -- token. Do not spawn the child.

Return the sum of all observations, widening each one before adding. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn option_terminator_total(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `option_terminator_total("")` gives `0`.
- `option_terminator_total("only")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1282/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1282`.

[Module reading](https://doc.rust-lang.org/std/process/)
