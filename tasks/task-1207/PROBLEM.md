# task-1207 — Line byte lengths · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Read a UTF-8 text stream line by line. Observe the byte length of each line after removing its LF or CRLF terminator. A final terminator ends the last line. Preserve interior empty lines.

Return distinct observations in ascending order. Use BufRead::lines with a Cursor over the input. During review, point to the operation that enforces its contract.

```rust
pub fn line_byte_lengths_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `line_byte_lengths_distinct("")` gives `vec![]`.
- `line_byte_lengths_distinct("a")` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1207/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1207`.

[Module reading](https://doc.rust-lang.org/std/io/)
