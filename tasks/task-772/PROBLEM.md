# task-772 — Index duplicate fields · shortest field with first tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn shortest_field_with_first_tie_slash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_slash(&["path/1/12", "path/123"])` gives `std::collections::BTreeMap::from([(String::from("1"), 1usize), (String::from("123"), 1usize)])`.
- `shortest_field_with_first_tie_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"])` gives `std::collections::BTreeMap::from([(String::from("1"), 1usize), (String::from("123"), 1usize), (String::from("hi"), 1usize), (String::from(""), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-772/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-772`.

[Module reading](https://doc.rust-lang.org/std/collections/)
