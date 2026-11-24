# task-502 — Split a slice · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn marks_removed_at_both_ends_hash(fields: (&str, &str)) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash(("outside[red#blue]ignored", "x"))` gives `8`.
- `marks_removed_at_both_ends_hash(("[猫###a]", "z#w"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-502/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-502`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
