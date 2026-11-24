# task-786 — Index duplicate fields · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn closing_marks_removed_equals(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals(&["72;12", "猫=12;note"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("猫=12"), 1usize)])`.
- `closing_marks_removed_equals(&["72;12", "猫=12;note", "hi=kit", "=é=a"])` gives `std::collections::BTreeMap::from([(String::from("72"), 1usize), (String::from("猫=12"), 1usize), (String::from("hi=kit"), 1usize), (String::from("=é=a"), 1usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-786/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-786`.

[Module reading](https://doc.rust-lang.org/std/collections/)
