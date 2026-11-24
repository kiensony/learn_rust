# task-2703 — CLI basenames · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each path's final normal component. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Return Some((minimum, maximum)) or None for an empty trace. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_basenames_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `cli_basenames_bounds("")` gives `None`.
- `cli_basenames_bounds("a.txt")` gives `Some((5u32, 5u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2703/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2703`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
