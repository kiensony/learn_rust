# task-2715 — CLI extensions · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file extension, omitting paths without an extension. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_extensions_changes(input: &str) -> Vec<u32>
```

## Public examples

- `cli_extensions_changes("")` gives `vec![]`.
- `cli_extensions_changes("a.txt")` gives `vec![3u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2715/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2715`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
