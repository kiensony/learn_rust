# task-760 — Index duplicate fields · first nonempty field (:)

Treat ':' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn first_nonempty_field_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_colon(&["hi:kit", ":é:a"])` gives `std::collections::BTreeMap::from([(String::from("hi"), 1usize), (String::from("é"), 1usize)])`.
- `first_nonempty_field_colon(&["hi:kit", ":é:a", "", ":"])` gives `std::collections::BTreeMap::from([(String::from("hi"), 1usize), (String::from("é"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-760/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-760`.

[Module reading](https://doc.rust-lang.org/std/collections/)
