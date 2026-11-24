# task-958 — Iterate over matching fields · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn prefix_before_final_mark_colon<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(vec!["hi:kit", ":é:a"])` gives `vec![2usize, 3usize]`.
- `prefix_before_final_mark_colon(vec![":é:a", ""])` gives `vec![3usize, 0usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-958/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-958`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
