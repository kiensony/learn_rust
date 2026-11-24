# task-984 — Iterate over matching fields · leading marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn leading_marks_removed_equals<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_equals(vec!["72;12", "猫=12;note"])` gives `vec![2usize, 6usize]`.
- `leading_marks_removed_equals(vec!["猫=12;note", "hi=kit"])` gives `vec![6usize, 6usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-984/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-984`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
