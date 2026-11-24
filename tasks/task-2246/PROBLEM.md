# task-2246 — Borrowed prefixes · threshold crossing

Scan a borrowed UTF-8 input with no per-field heap allocation. Observe byte lengths of the ASCII-whitespace-separated words. Allocate only storage needed for the observations or result.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use borrowed slices or indices into the original input. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn borrowed_prefixes_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `borrowed_prefixes_threshold_crossing("a b c\n\r\nup\tdown", 0)` gives `Some((0, 1))`.
- `borrowed_prefixes_threshold_crossing("", 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2246/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2246`.

[Module reading](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
