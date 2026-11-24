# task-1135 — Read a trait object · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn marks_removed_at_both_ends_equals(source: &dyn Source) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(&"72;12")` gives `2`.
- `marks_removed_at_both_ends_equals(&String::from("猫=12;note"))` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1135/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1135`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
