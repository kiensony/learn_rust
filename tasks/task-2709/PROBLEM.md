# task-2709 — CLI basenames · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each path's final normal component. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_basenames_longest_run(input: &str) -> u64
```

## Public examples

- `cli_basenames_longest_run("")` gives `0`.
- `cli_basenames_longest_run("a.txt")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2709/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2709`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
