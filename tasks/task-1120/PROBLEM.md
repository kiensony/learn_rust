# task-1120 — Read a trait object · shortest field with first tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn shortest_field_with_first_tie_slash(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_slash(&"path/1/12")` gives `1`.
- `shortest_field_with_first_tie_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1120/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1120`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
