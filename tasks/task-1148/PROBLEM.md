# task-1148 — Read a trait object · first nonempty field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn first_nonempty_field_hash(source: &dyn Source) -> usize
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `first_nonempty_field_hash(&"outside[red#blue]ignored")` gives `3`.
- `first_nonempty_field_hash(&"unclosed[cat")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1148/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1148`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
