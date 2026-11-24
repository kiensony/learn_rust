# task-787 — Index duplicate fields · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn marks_removed_at_both_ends_equals(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(&["72;12", "猫=12;note"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("猫=12"), 1usize)])`.
- `marks_removed_at_both_ends_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("猫=12"), 1usize), (String::from("hi=kit"), 1usize), (String::from("é=a"), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-787/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-787`.

[Module reading](https://doc.rust-lang.org/std/collections/)
