# task-1289 — Option terminator · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect the byte length of every argument following the first standalone -- token. Do not spawn the child.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn option_terminator_longest_run(input: &str) -> u64
```

## Public examples

- `option_terminator_longest_run("")` gives `0`.
- `option_terminator_longest_run("only")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1289/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1289`.

[Module reading](https://doc.rust-lang.org/std/process/)
