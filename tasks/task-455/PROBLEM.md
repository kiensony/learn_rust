# task-455 — Edit the caller’s label · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn first_nonempty_field_hash(text: &mut String) -> ()
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `{ let mut data = String::from("outside[red#blue]ignored"); first_nonempty_field_hash(&mut data); data }` gives `"red"`.
- `{ let mut data = String::from("[猫###a]"); first_nonempty_field_hash(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-455/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-455`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
