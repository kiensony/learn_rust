# task-1141 — Read a trait object · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn leading_field_hash(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash(&"outside[red#blue]ignored")` gives `3`.
- `leading_field_hash(&"unclosed[cat")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1141/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1141`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
