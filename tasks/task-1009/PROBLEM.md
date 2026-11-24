# task-1009 — Accept generic text · first nonempty field (:)

Treat ':' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn first_nonempty_field_colon<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_colon(["hi:kit", ":é:a"])` gives `[2, 2]`.
- `first_nonempty_field_colon([String::from(":é:a"), String::from("")])` gives `[2, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1009/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1009`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
