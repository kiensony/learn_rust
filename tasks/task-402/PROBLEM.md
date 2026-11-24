# task-402 — Inspect a borrowed label · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn marks_removed_at_both_ends_hash(text: &String) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash(&String::from("outside[red#blue]ignored"))` gives `8`.
- `marks_removed_at_both_ends_hash(&String::from("[猫###a]"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-402/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-402`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
