# task-759 — Index duplicate fields · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Select one field from every record and count occurrences in a BTreeMap. Keep the empty key if it appears. Keys own their text.

```rust
pub fn prefix_before_final_mark_colon(records: &[&str]) -> std::collections::BTreeMap<String, usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(&["hi:kit", ":é:a"])` gives `std::collections::BTreeMap::from([(String::from("hi"), 1usize), (String::from(":é"), 1usize)])`.
- `prefix_before_final_mark_colon(&["hi:kit", ":é:a", "", ":"])` gives `std::collections::BTreeMap::from([(String::from("hi"), 1usize), (String::from(":é"), 1usize), (String::from(""), 2usize)])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-759/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-759`.

[Module reading](https://doc.rust-lang.org/std/collections/)
