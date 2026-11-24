# task-1144 — Read a trait object · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn closing_marks_removed_hash(source: &dyn Source) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `closing_marks_removed_hash(&"outside[red#blue]ignored")` gives `8`.
- `closing_marks_removed_hash(&String::from("[猫###a]"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1144/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1144`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
