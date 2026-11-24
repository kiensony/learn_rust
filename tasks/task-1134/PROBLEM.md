# task-1134 — Read a trait object · closing marks removed (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn closing_marks_removed_equals(source: &dyn Source) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_equals(&"72;12")` gives `2`.
- `closing_marks_removed_equals(&String::from("猫=12;note"))` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1134/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1134`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
