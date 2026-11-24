# task-401 — Inspect a borrowed label · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn closing_marks_removed_hash(text: &String) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash(&String::from("outside[red#blue]ignored"))` gives `8`.
- `closing_marks_removed_hash(&String::from("[猫###a]"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-401/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-401`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
