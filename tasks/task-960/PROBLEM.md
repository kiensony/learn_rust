# task-960 — Iterate over matching fields · longest field with last tie (:)

Treat ':' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn longest_field_with_last_tie_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_colon(vec!["hi:kit", ":é:a"])` gives `vec![3usize, 2usize]`.
- `longest_field_with_last_tie_colon(vec![":é:a", ""])` gives `vec![2usize, 0usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-960/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-960`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
