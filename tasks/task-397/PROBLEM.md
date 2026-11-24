# task-397 — Inspect a borrowed label · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn shortest_field_with_first_tie_equals(text: &String) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals(&String::from("72;12"))` gives `2`.
- `shortest_field_with_first_tie_equals(&String::from("=é=a"))` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-397/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-397`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
