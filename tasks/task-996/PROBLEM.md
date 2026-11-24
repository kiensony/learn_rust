# task-996 — Iterate over matching fields · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn marks_removed_at_both_ends_hash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash(vec!["outside[red#blue]ignored", "[猫###a]"])` gives `vec![8usize, 7usize]`.
- `marks_removed_at_both_ends_hash(vec!["[猫###a]", "unclosed[cat"])` gives `vec![7usize, 0usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-996/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-996`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
