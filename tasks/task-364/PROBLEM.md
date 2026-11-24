# task-364 — Inspect a borrowed label · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn prefix_before_final_mark_colon(text: &String) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(&String::from("hi:kit"))` gives `2`.
- `prefix_before_final_mark_colon(&String::from(":é:a"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-364/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-364`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
