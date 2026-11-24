# task-605 — Return a borrowed field · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn longest_field_with_last_tie_hash<'a>(text: &'a str) -> &'a str
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash("outside[red#blue]ignored")` gives `"blue"`.
- `longest_field_with_last_tie_hash("[猫###a]")` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-605/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-605`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
