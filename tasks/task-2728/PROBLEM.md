# task-2728 — CLI path depth · steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the count of normal path components in each path, excluding root, dot and dot-dot. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the absolute differences between each adjacent pair, in order. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_path_depth_steps(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cli_path_depth_steps("")` gives `vec![]`.
- `cli_path_depth_steps("a.txt")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2728/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2728`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
