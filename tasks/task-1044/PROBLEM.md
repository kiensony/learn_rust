# task-1044 — Accept generic text · leading marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn leading_marks_removed_hash<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_hash(["outside[red#blue]ignored", "[猫###a]"])` gives `[8, 7]`.
- `leading_marks_removed_hash([String::from("[猫###a]"), String::from("unclosed[cat")])` gives `[7, 0]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1044/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1044`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
