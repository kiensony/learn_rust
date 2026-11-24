# task-861 — Expose a module function · longest field with last tie (:)

Treat ':' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn longest_field_with_last_tie_colon(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `longest_field_with_last_tie_colon("hi:kit")` gives `5`.
- `longest_field_with_last_tie_colon(":é:a")` gives `4`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-861/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-861`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
