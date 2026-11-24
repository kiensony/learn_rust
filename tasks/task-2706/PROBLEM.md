# task-2706 — CLI basenames · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each path's final normal component. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Count adjacent pairs for which the second observation is strictly smaller. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_basenames_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cli_basenames_descending_steps("")` gives `0`.
- `cli_basenames_descending_steps("a.txt")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2706/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2706`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
