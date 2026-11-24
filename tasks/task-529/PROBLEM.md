# task-529 — Measure Unicode text · closing field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn closing_field_pipe(text: &str) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_pipe("first\n123")` gives `3`.
- `closing_field_pipe("|é|a")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-529/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-529`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
