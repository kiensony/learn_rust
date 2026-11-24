# task-773 — Index duplicate fields · leading field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn leading_field_pipe(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_field_pipe(&["first\n123", "header\n猫|123"])` gives `std::collections::BTreeMap::from([(String::from("123"), 1usize), (String::from("猫"), 1usize)])`.
- `leading_field_pipe(&["first\n123", "header\n猫|123", "hi|kit", "|é|a"])` gives `std::collections::BTreeMap::from([(String::from("123"), 1usize), (String::from("猫"), 1usize), (String::from("hi"), 1usize), (String::from(""), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-773/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-773`.

[Module reading](https://doc.rust-lang.org/std/collections/)
