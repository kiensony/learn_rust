# task-457 — Edit the caller’s label · shortest field with first tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn shortest_field_with_first_tie_hash(text: &mut String) -> ()
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `{ let mut data = String::from("outside[red#blue]ignored"); shortest_field_with_first_tie_hash(&mut data); data }` gives `"red"`.
- `{ let mut data = String::from("[猫###a]"); shortest_field_with_first_tie_hash(&mut data); data }` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-457/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-457`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
