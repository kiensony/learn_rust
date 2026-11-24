# task-1017 — Accept generic text · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn remainder_after_first_mark_slash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_slash(["path/1/12", "path/123"])` gives `[2, 0]`.
- `remainder_after_first_mark_slash([String::from("path/123"), String::from("hi/kit")])` gives `[0, 3]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1017/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1017`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
