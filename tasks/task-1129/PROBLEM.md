# task-1129 — Read a trait object · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn longest_field_with_last_tie_pipe(source: &dyn Source) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `longest_field_with_last_tie_pipe(&"first\n123")` gives `3`.
- `longest_field_with_last_tie_pipe(&String::from("|é|a"))` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1129/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1129`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
