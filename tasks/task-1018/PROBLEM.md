# task-1018 — Accept generic text · prefix before final mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn prefix_before_final_mark_slash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_slash(["path/1/12", "path/123"])` gives `[1, 0]`.
- `prefix_before_final_mark_slash([String::from("path/123"), String::from("hi/kit")])` gives `[0, 2]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1018/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1018`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
