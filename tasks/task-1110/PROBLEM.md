# task-1110 — Read a trait object · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn shortest_field_with_first_tie_colon(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `shortest_field_with_first_tie_colon(&"hi:kit")` gives `2`.
- `shortest_field_with_first_tie_colon(&String::from(":é:a"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1110/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1110`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
