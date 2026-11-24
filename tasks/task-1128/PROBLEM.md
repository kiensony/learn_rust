# task-1128 — Read a trait object · first nonempty field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn first_nonempty_field_pipe(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_pipe(&"first\n123")` gives `3`.
- `first_nonempty_field_pipe(&"hi|kit")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1128/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1128`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
