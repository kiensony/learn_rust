# task-2217 — Borrowed offsets · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe zero-based byte offsets of LF terminators. Allocate only storage needed for the observations or result.

Return distinct observations in ascending order. Use borrowed slices or indices into the original input. During review, point to the operation that enforces its contract.

```rust
pub fn borrowed_offsets_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `borrowed_offsets_distinct("")` gives `vec![]`.
- `borrowed_offsets_distinct("a")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2217/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2217`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
