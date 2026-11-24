# task-1101 — Read a trait object · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn leading_field_colon(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_colon(&"hi:kit")` gives `2`.
- `leading_field_colon(&String::from(":é:a"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1101/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1101`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
