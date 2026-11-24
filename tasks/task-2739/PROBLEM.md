# task-2739 — CLI Rust sources · longest run

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the zero-based argument index of every path whose extension is exactly rs. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_rust_sources_longest_run(input: &str) -> u64
```

## Public examples

- `cli_rust_sources_longest_run("")` gives `0`.
- `cli_rust_sources_longest_run("a.txt")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2739/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2739`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
