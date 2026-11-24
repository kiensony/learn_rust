# task-1026 — Accept generic text · marks removed at both ends (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn marks_removed_at_both_ends_pipe<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_pipe(["first\n123", "header\n猫|123"])` gives `[3, 7]`.
- `marks_removed_at_both_ends_pipe([String::from("header\n猫|123"), String::from("hi|kit")])` gives `[7, 6]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1026/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1026`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
