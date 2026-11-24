# task-2721 — CLI path depth · trace

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the count of normal path components in each path, excluding root, dot and dot-dot. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the observations in input order, including duplicates. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_path_depth_trace(input: &str) -> Vec<u32>
```

## Public examples

- `cli_path_depth_trace("")` gives `vec![]`.
- `cli_path_depth_trace("a.txt")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2721/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2721`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
