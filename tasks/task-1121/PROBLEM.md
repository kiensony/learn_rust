# task-1121 — Read a trait object · leading field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn leading_field_pipe(source: &dyn Source) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_field_pipe(&"first\n123")` gives `3`.
- `leading_field_pipe(&"hi|kit")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1121/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1121`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
