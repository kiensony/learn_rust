# task-1293 — Quiet option positions · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect zero-based positions of exact -q arguments, wherever they appear. Do not spawn the child.

Return Some((minimum, maximum)) or None for an empty trace. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn quiet_option_positions_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `quiet_option_positions_bounds("")` gives `None`.
- `quiet_option_positions_bounds("only")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1293/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1293`.

[Module reading](https://doc.rust-lang.org/std/process/)
