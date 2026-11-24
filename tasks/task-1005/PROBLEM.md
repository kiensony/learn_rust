# task-1005 — Accept generic text · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn closing_marks_removed_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon(["hi:kit", ":é:a"])` gives `[6, 5]`.
- `closing_marks_removed_colon([String::from(":é:a"), String::from("")])` gives `[5, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1005/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1005`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
