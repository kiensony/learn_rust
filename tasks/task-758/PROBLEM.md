# task-758 — Index duplicate fields · marks removed at both ends (:)

Treat ':' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn marks_removed_at_both_ends_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_colon(&["hi:kit", ":é:a"])` gives `std::collections::BTreeMap::from([(String::from("hi:kit"), 1usize), (String::from("é:a"), 1usize)])`.
- `marks_removed_at_both_ends_colon(&["hi:kit", ":é:a", "", ":"])` gives `std::collections::BTreeMap::from([(String::from("hi:kit"), 1usize), (String::from("é:a"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-758/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-758`.

[Module reading](https://doc.rust-lang.org/std/collections/)
