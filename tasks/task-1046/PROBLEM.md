# task-1046 — Accept generic text · marks removed at both ends (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn marks_removed_at_both_ends_hash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_hash(["outside[red#blue]ignored", "[猫###a]"])` gives `[8, 7]`.
- `marks_removed_at_both_ends_hash([String::from("[猫###a]"), String::from("unclosed[cat")])` gives `[7, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1046/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1046`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
