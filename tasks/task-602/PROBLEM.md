# task-602 — Return a borrowed field · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn remainder_after_first_mark_hash<'a>(text: &'a str) -> &'a str
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `remainder_after_first_mark_hash("outside[red#blue]ignored")` gives `"blue"`.
- `remainder_after_first_mark_hash("[猫###a]")` gives `"##a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-602/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-602`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
