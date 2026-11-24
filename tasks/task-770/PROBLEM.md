# task-770 — Index duplicate fields · first nonempty field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn first_nonempty_field_slash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `first_nonempty_field_slash(&["path/1/12", "path/123"])` gives `std::collections::BTreeMap::from([(String::from("1"), 1usize), (String::from("123"), 1usize)])`.
- `first_nonempty_field_slash(&["path/1/12", "path/123", "hi/kit", "/é/a"])` gives `std::collections::BTreeMap::from([(String::from("1"), 1usize), (String::from("123"), 1usize), (String::from("hi"), 1usize), (String::from("é"), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-770/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-770`.

[Module reading](https://doc.rust-lang.org/std/collections/)
