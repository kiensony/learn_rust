# task-2743 — CLI stems · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file stem, preserving earlier extensions in compound names. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return Some((minimum, maximum)) or None for an empty trace. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_stems_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `cli_stems_bounds("")` gives `None`.
- `cli_stems_bounds("a.txt")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2743/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2743`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
