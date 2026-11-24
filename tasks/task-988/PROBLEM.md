# task-988 — Iterate over matching fields · prefix before final mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn prefix_before_final_mark_equals<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `prefix_before_final_mark_equals(vec!["72;12", "猫=12;note"])` gives `vec![0usize, 3usize]`.
- `prefix_before_final_mark_equals(vec!["猫=12;note", "hi=kit"])` gives `vec![3usize, 2usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-988/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-988`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
