# task-892 — Expose a module function · shortest field with first tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn shortest_field_with_first_tie_equals(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `shortest_field_with_first_tie_equals("72;12")` gives `4`.
- `shortest_field_with_first_tie_equals("=é=a")` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-892/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-892`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
