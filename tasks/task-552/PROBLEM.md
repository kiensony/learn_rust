# task-552 — Measure Unicode text · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn marks_removed_at_both_ends_hash(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash("outside[red#blue]ignored")` gives `8`.
- `marks_removed_at_both_ends_hash("[猫###a]")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-552/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-552`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
