# task-977 — Iterate over matching fields · remainder after first mark (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn remainder_after_first_mark_pipe<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_pipe(vec!["first\n123", "header\n猫|123"])` gives `vec![0usize, 3usize]`.
- `remainder_after_first_mark_pipe(vec!["header\n猫|123", "hi|kit"])` gives `vec![3usize, 3usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-977/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-977`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
