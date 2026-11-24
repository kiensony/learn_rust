# task-1036 — Accept generic text · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn marks_removed_at_both_ends_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(["72;12", "猫=12;note"])` gives `[2, 6]`.
- `marks_removed_at_both_ends_equals([String::from("猫=12;note"), String::from("hi=kit")])` gives `[6, 6]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1036/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1036`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
