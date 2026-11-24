# task-528 — Measure Unicode text · leading field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. After selecting the field, count its Unicode scalar values. An emoji occupies one scalar value in the examples.

```rust
pub fn leading_field_pipe(text: &str) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_field_pipe("first\n123")` gives `3`.
- `leading_field_pipe("header\n猫|123")` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-528/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-528`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
