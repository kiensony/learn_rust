# task-792 — Index duplicate fields · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn shortest_field_with_first_tie_equals(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals(&["72;12", "猫=12;note"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("12"), 1usize)])`.
- `shortest_field_with_first_tie_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("12"), 1usize), (String::from("hi"), 1usize), (String::from(""), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-792/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-792`.

[Module reading](https://doc.rust-lang.org/std/collections/)
