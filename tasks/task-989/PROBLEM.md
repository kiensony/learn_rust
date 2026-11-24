# task-989 — Iterate over matching fields · first nonempty field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Produce a Vec of one selected byte length per record in input order using an iterator adapter. Keep duplicates and zero lengths.

```rust
pub fn first_nonempty_field_equals<'a>(records: impl IntoIterator<Item = &'a str>) -> Vec<usize>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_equals(vec!["72;12", "猫=12;note"])` gives `vec![2usize, 3usize]`.
- `first_nonempty_field_equals(vec!["猫=12;note", "hi=kit"])` gives `vec![3usize, 2usize]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-989/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-989`.

[Module reading](https://doc.rust-lang.org/book/ch13-02-iterators.html)
