# task-601 — Return a borrowed field · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn marks_removed_at_both_ends_hash<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash("outside[red#blue]ignored")` gives `"red#blue"`.
- `marks_removed_at_both_ends_hash("[猫###a]")` gives `"猫###a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-601/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-601`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
