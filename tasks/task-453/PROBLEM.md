# task-453 — Edit the caller’s label · remainder after first mark (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn remainder_after_first_mark_hash(text: &mut String) -> ()
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `{ let mut data = String::from("outside[red#blue]ignored"); remainder_after_first_mark_hash(&mut data); data }` gives `"blue"`.
- `{ let mut data = String::from("[猫###a]"); remainder_after_first_mark_hash(&mut data); data }` gives `"##a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-453/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-453`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
