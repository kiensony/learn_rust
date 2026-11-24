# task-795 — Index duplicate fields · leading marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn leading_marks_removed_hash(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_hash(&["outside[red#blue]ignored", "[猫###a]"])` gives `std::collections::BTreeMap::from([(String::from("red#blue"), 1usize), (String::from("猫###a"), 1usize)])`.
- `leading_marks_removed_hash(&["outside[red#blue]ignored", "[猫###a]", "unclosed[cat", "[]"])` gives `std::collections::BTreeMap::from([(String::from("red#blue"), 1usize), (String::from("猫###a"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-795/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-795`.

[Module reading](https://doc.rust-lang.org/std/collections/)
