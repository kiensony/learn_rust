# task-764 — Index duplicate fields · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn closing_field_slash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_field_slash(&["path/1/12", "path/123"])` gives `std::collections::BTreeMap::from([(String::from("12"), 1usize), (String::from("123"), 1usize)])`.
- `closing_field_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"])` gives `std::collections::BTreeMap::from([(String::from("12"), 1usize), (String::from("123"), 1usize), (String::from("kit"), 1usize), (String::from("a"), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-764/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-764`.

[Module reading](https://doc.rust-lang.org/std/collections/)
