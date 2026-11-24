# task-1099 — Use an associated type · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn longest_field_with_last_tie_hash(packet: &impl Packet) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash(&"outside[red#blue]ignored")` gives `4`.
- `longest_field_with_last_tie_hash(&String::from("[猫###a]"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1099/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1099`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
