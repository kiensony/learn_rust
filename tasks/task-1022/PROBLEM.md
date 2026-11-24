# task-1022 — Accept generic text · leading field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn leading_field_pipe<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_field_pipe(["first\n123", "header\n猫|123"])` gives `[3, 3]`.
- `leading_field_pipe([String::from("header\n猫|123"), String::from("hi|kit")])` gives `[3, 2]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1022/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1022`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
