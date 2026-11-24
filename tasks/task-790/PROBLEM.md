# task-790 — Index duplicate fields · first nonempty field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn first_nonempty_field_equals(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_equals(&["72;12", "猫=12;note"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("猫"), 1usize)])`.
- `first_nonempty_field_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("猫"), 1usize), (String::from("hi"), 1usize), (String::from("é"), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-790/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-790`.

[Module reading](https://doc.rust-lang.org/std/collections/)
