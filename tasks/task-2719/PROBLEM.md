# task-2719 — CLI extensions · longest run

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file extension, omitting paths without an extension. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_extensions_longest_run(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cli_extensions_longest_run("")` gives `0`.
- `cli_extensions_longest_run("a.txt")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2719/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2719`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
