# task-1295 — Quiet option positions · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect zero-based positions of exact -q arguments, wherever they appear. Do not spawn the child.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn quiet_option_positions_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `quiet_option_positions_changes("")` gives `vec![]`.
- `quiet_option_positions_changes("only")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1295/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1295`.

[Module reading](https://doc.rust-lang.org/std/process/)
