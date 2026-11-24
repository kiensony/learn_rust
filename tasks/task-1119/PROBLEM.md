# task-1119 — Read a trait object · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn longest_field_with_last_tie_slash(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `longest_field_with_last_tie_slash(&"path/1/12")` gives `2`.
- `longest_field_with_last_tie_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1119/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1119`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
