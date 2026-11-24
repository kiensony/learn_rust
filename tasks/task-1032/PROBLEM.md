# task-1032 — Accept generic text · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept both String and &str elements through AsRef<str>. Return the selected byte lengths in an array of the same compile-time length.

```rust
pub fn leading_field_equals<T: AsRef<str>, const N: usize>(records: [T; N]) -> [usize; N]
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_equals(["72;12", "猫=12;note"])` gives `[2, 3]`.
- `leading_field_equals([String::from("猫=12;note"), String::from("hi=kit")])` gives `[3, 2]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1032/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1032`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
