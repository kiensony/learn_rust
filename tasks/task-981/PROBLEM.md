# task-981 — Iterate over matching fields · shortest field with first tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn shortest_field_with_first_tie_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_pipe(vec!["first\n123", "header\n猫|123"])` gives `vec![3usize, 3usize]`.
- `shortest_field_with_first_tie_pipe(vec!["header\n猫|123", "hi|kit"])` gives `vec![3usize, 2usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-981/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-981`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
