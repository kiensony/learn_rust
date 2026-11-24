# task-1098 — Use an associated type · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn first_nonempty_field_hash(packet: &impl Packet) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash(&"outside[red#blue]ignored")` gives `3`.
- `first_nonempty_field_hash(&"unclosed[cat")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1098/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1098`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
