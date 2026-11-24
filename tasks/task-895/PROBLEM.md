# task-895 — Expose a module function · leading marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn leading_marks_removed_hash(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_hash("outside[red#blue]ignored")` gives `10`.
- `leading_marks_removed_hash("[猫###a]")` gives `9`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-895/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-895`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
