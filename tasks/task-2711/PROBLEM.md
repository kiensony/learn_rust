# task-2711 — CLI extensions · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file extension, omitting paths without an extension. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the observations in input order, including duplicates. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_extensions_trace(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `cli_extensions_trace("")` gives `vec![]`.
- `cli_extensions_trace("a.txt")` gives `vec![3u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2711/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2711`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
