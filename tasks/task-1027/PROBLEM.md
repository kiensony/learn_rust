# task-1027 — Accept generic text · remainder after first mark (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn remainder_after_first_mark_pipe<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_pipe(["first\n123", "header\n猫|123"])` gives `[0, 3]`.
- `remainder_after_first_mark_pipe([String::from("header\n猫|123"), String::from("hi|kit")])` gives `[3, 3]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1027/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1027`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
