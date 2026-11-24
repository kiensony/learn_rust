# task-1294 — Quiet option positions · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect zero-based positions of exact -q arguments, wherever they appear. Do not spawn the child.

Keep the first observation and every later one strictly above all earlier observations. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn quiet_option_positions_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `quiet_option_positions_peaks("")` gives `vec![]`.
- `quiet_option_positions_peaks("only")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1294/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1294`.

[Module reading](https://doc.rust-lang.org/std/process/)
