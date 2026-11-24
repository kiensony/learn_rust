# task-755 — Index duplicate fields · closing field (:)

Treat ':' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn closing_field_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_colon(&["hi:kit", ":é:a"])` gives `std::collections::BTreeMap::from([(String::from("kit"), 1usize), (String::from("a"), 1usize)])`.
- `closing_field_colon(&["hi:kit", ":é:a", "", ":"])` gives `std::collections::BTreeMap::from([(String::from("kit"), 1usize), (String::from("a"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-755/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-755`.

[Module reading](https://doc.rust-lang.org/std/collections/)
