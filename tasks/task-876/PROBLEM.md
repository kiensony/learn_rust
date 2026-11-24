# task-876 — Expose a module function · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn closing_marks_removed_pipe(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `closing_marks_removed_pipe("first\n123")` gives `5`.
- `closing_marks_removed_pipe("header\n猫|123")` gives `9`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-876/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-876`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
