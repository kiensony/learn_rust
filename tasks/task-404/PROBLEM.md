# task-404 — Check a borrowed prefix

Decide whether the complete header is the prefix of the supplied bytes. An empty header matches every input. Neither slice changes.

```rust
pub fn has_header(bytes: &[u8], header: &[u8]) -> bool
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `has_header(&[2, 3, 4], &[2, 3])` gives `true`.
- `has_header(&[2, 4], &[4])` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-404/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-404`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
