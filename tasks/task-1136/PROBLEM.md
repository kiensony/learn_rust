# task-1136 — Read a trait object · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn remainder_after_first_mark_equals(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals(&"72;12")` gives `0`.
- `remainder_after_first_mark_equals(&String::from("猫=12;note"))` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1136/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1136`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
