# task-1030 — Accept generic text · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn longest_field_with_last_tie_pipe<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(["first\n123", "header\n猫|123"])` gives `[3, 3]`.
- `longest_field_with_last_tie_pipe(["hi|kit"])` gives `[3]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1030/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1030`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
