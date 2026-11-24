# task-992 — Iterate over matching fields · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn leading_field_hash<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash(vec!["outside[red#blue]ignored", "[猫###a]"])` gives `vec![3usize, 3usize]`.
- `leading_field_hash(vec!["[猫###a]", "unclosed[cat"])` gives `vec![3usize, 0usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-992/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-992`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
