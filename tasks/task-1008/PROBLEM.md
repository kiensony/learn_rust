# task-1008 — Accept generic text · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn prefix_before_final_mark_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon(["hi:kit", ":é:a"])` gives `[2, 3]`.
- `prefix_before_final_mark_colon([String::from(":é:a"), String::from("")])` gives `[3, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1008/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1008`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
