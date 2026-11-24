# task-703 — Route an event · longest field with last tie (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. For Data return Some(selected byte length), for Heartbeat return Some(0), and for End return None. Match all three variants explicitly.

```rust
pub fn longest_field_with_last_tie_hash(event: Event<'_>) -> Option<usize>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `longest_field_with_last_tie_hash(Event::Data("outside[red#blue]ignored"))` gives `Some(4)`.
- `longest_field_with_last_tie_hash(Event::Data("[猫###a]"))` gives `Some(3)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-703/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-703`.

[Module reading](https://doc.rust-lang.org/book/ch06-02-match.html)
