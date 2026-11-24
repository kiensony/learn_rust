# task-497 — Split a slice · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn shortest_field_with_first_tie_equals(fields: (&str, &str)) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals(("72;12", "x"))` gives `2`.
- `shortest_field_with_first_tie_equals(("=é=a", "z=w"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-497/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-497`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
