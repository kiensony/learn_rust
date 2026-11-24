# task-1020 — Accept generic text · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn longest_field_with_last_tie_slash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash(["path/1/12", "path/123"])` gives `[2, 3]`.
- `longest_field_with_last_tie_slash([String::from("path/123"), String::from("hi/kit")])` gives `[3, 3]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1020/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1020`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
