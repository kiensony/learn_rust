# task-1003 — Accept generic text · closing field (:)

Treat ':' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn closing_field_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_colon(["hi:kit", ":é:a"])` gives `[3, 1]`.
- `closing_field_colon([String::from(":é:a"), String::from("")])` gives `[1, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1003/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1003`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
