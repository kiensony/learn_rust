# task-991 — Iterate over matching fields · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn shortest_field_with_first_tie_equals<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals(vec!["72;12", "猫=12;note"])` gives `vec![2usize, 2usize]`.
- `shortest_field_with_first_tie_equals(vec![])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-991/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-991`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
