# task-1285 — Option terminator · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte length of every argument following the first standalone -- token. Do not spawn the child.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn option_terminator_changes(input: &str) -> Vec<u32>
```

## Public examples

- `option_terminator_changes("")` gives `vec![]`.
- `option_terminator_changes("only")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1285/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1285`.

[Module reading](https://doc.rust-lang.org/std/process/)
