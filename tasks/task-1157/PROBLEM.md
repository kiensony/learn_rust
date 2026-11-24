# task-1157 — Borrow a view · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller. Whenever the selected text is empty, return the caller’s fallback slice.

```rust
pub fn prefix_before_final_mark_colon<'a>(text: &'a str, fallback: &'a str) -> &'a str
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon("hi:kit", "backup")` gives `"hi"`.
- `prefix_before_final_mark_colon(":é:a", "backup")` gives `":é"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1157/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1157`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
