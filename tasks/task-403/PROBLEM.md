# task-403 — Inspect a borrowed label · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn remainder_after_first_mark_hash(text: &String) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash(&String::from("outside[red#blue]ignored"))` gives `4`.
- `remainder_after_first_mark_hash(&String::from("[猫###a]"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-403/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-403`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
