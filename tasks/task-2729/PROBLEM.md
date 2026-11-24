# task-2729 — CLI path depth · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the count of normal path components in each path, excluding root, dot and dot-dot. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_path_depth_longest_run(input: &str) -> u64
```

## Public examples

- `cli_path_depth_longest_run("")` gives `0`.
- `cli_path_depth_longest_run("a.txt")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2729/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2729`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
