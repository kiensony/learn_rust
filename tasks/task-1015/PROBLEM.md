# task-1015 — Accept generic text · closing marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn closing_marks_removed_slash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_slash(["path/1/12", "path/123"])` gives `[4, 3]`.
- `closing_marks_removed_slash([String::from("path/123"), String::from("hi/kit")])` gives `[3, 6]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1015/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1015`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
