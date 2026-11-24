# task-1277 — Explicit levels · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect numeric values of all --level=N tokens whose N parses as u8; ignore other tokens. Do not spawn the child.

Return distinct observations in ascending order. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn explicit_levels_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `explicit_levels_distinct("")` gives `vec![]`.
- `explicit_levels_distinct("only")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1277/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1277`.

[Module reading](https://doc.rust-lang.org/std/process/)
