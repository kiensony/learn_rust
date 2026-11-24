# task-1262 — Positional options · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte lengths of arguments whose first character is not a hyphen. Do not spawn the child.

Return the sum of all observations, widening each one before adding. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn positional_options_total(input: &str) -> u64
```

## Public examples

- `positional_options_total("")` gives `0`.
- `positional_options_total("only")` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1262/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1262`.

[Module reading](https://doc.rust-lang.org/std/process/)
