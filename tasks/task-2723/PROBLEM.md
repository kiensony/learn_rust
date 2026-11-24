# task-2723 — CLI path depth · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the count of normal path components in each path, excluding root, dot and dot-dot. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return Some((minimum, maximum)) or None for an empty trace. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_path_depth_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cli_path_depth_bounds("")` gives `None`.
- `cli_path_depth_bounds("a.txt")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2723/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2723`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
