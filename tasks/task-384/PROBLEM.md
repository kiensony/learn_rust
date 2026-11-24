# task-384 — Measure two views

Count matching mirror pairs in the slice. Pair its first byte with its last, its second with its second-last, and so on. A center byte never forms a pair.

```rust
pub fn mirror_count(data: &[u8]) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `mirror_count(&[1, 2, 1])` gives `1`.
- `mirror_count(&[3, 4, 4, 3])` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-384/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-384`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
