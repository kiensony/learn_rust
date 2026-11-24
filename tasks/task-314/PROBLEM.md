# task-314 — Consume a label · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Take ownership of the input and return the selected text as an owned String. The caller must be able to move the result into a new binding.

```rust
pub fn prefix_before_final_mark_colon(mut text: String) -> String
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(String::from("hi:kit"))` gives `"hi"`.
- `prefix_before_final_mark_colon(String::from(":é:a"))` gives `":é"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-314/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-314`.

[Module reading](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
