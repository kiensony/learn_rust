# task-553 — Measure Unicode text · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn remainder_after_first_mark_hash(text: &str) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash("outside[red#blue]ignored")` gives `4`.
- `remainder_after_first_mark_hash("[猫###a]")` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-553/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-553`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
