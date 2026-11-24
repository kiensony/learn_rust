# task-1040 — Accept generic text · longest field with last tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn longest_field_with_last_tie_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_equals(["72;12", "猫=12;note"])` gives `[2, 3]`.
- `longest_field_with_last_tie_equals([String::from("猫=12;note"), String::from("hi=kit")])` gives `[3, 3]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1040/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1040`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
