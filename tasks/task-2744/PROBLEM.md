# task-2744 — CLI stems · peaks

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

A file-listing command accepts ASCII whitespace-separated portable paths. Inputs use normal ASCII component names, optional / separators, . at the start, and .. followed by a normal name. Do all normalization lexically. Observe the byte length of each file stem, preserving earlier extensions in compound names. Avoid filesystem access so missing paths and machine-specific roots cannot affect the result.

Keep the first observation and every later one strictly above all earlier observations. Use std::path::Path component inspection. During review, point to the operation that enforces its contract.

```rust
pub fn cli_stems_peaks(input: &str) -> Vec<u32>
```

## Public examples

- `cli_stems_peaks("")` gives `vec![]`.
- `cli_stems_peaks("a.txt")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2744/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2744`.

[Module reading](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
