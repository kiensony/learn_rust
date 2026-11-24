# task-2705 — CLI basenames · changes

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each path's final normal component. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_basenames_changes(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cli_basenames_changes("")` gives `vec![]`.
- `cli_basenames_changes("a.txt")` gives `vec![5u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2705/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2705`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
