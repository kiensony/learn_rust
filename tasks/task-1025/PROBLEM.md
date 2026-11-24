# task-1025 — Accept generic text · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn closing_marks_removed_pipe<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_marks_removed_pipe(["first\n123", "header\n猫|123"])` gives `[3, 7]`.
- `closing_marks_removed_pipe([String::from("header\n猫|123"), String::from("hi|kit")])` gives `[7, 6]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1025/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1025`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
