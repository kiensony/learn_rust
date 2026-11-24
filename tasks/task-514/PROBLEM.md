# task-514 — Measure Unicode text · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn prefix_before_final_mark_colon(text: &str) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon("hi:kit")` gives `2`.
- `prefix_before_final_mark_colon("")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-514/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-514`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
