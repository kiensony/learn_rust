# task-1139 — Read a trait object · longest field with last tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn longest_field_with_last_tie_equals(source: &dyn Source) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_equals(&"72;12")` gives `2`.
- `longest_field_with_last_tie_equals(&String::from("猫=12;note"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1139/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1139`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
