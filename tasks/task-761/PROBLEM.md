# task-761 — Index duplicate fields · longest field with last tie (:)

Treat ':' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn longest_field_with_last_tie_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_colon(&["hi:kit", ":é:a"])` gives `std::collections::BTreeMap::from([(String::from("kit"), 1usize), (String::from("é"), 1usize)])`.
- `longest_field_with_last_tie_colon(&["hi:kit", ":é:a", "", ":"])` gives `std::collections::BTreeMap::from([(String::from("kit"), 1usize), (String::from("é"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-761/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-761`.

[Module reading](https://doc.rust-lang.org/std/collections/)
