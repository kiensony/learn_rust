# task-2207 — Borrowed prefixes · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe byte lengths of the ASCII-whitespace-separated words. Allocate only storage needed for the observations or result.

Return distinct observations in ascending order. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_prefixes_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `borrowed_prefixes_distinct("")` gives `vec![]`.
- `borrowed_prefixes_distinct("a")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2207/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2207`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
