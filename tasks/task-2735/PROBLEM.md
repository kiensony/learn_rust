# task-2735 — CLI Rust sources · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the zero-based argument index of every path whose extension is exactly rs. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_rust_sources_changes(input: &str) -> Vec<u32>
```

## Public examples

- `cli_rust_sources_changes("")` gives `vec![]`.
- `cli_rust_sources_changes("a.txt")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2735/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2735`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
