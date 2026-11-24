# task-1118 — Read a trait object · first nonempty field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn first_nonempty_field_slash(source: &dyn Source) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `first_nonempty_field_slash(&"path/1/12")` gives `1`.
- `first_nonempty_field_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1118/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1118`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
