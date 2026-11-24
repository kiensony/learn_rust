# task-980 — Iterate over matching fields · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn longest_field_with_last_tie_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(vec!["first\n123", "header\n猫|123"])` gives `vec![3usize, 3usize]`.
- `longest_field_with_last_tie_pipe(vec![])` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-980/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-980`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
