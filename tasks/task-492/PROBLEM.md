# task-492 — Split a slice · marks removed at both ends (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn marks_removed_at_both_ends_equals(fields: (&str, &str)) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `marks_removed_at_both_ends_equals(("72;12", "x"))` gives `2`.
- `marks_removed_at_both_ends_equals(("猫=12;note", "z=w"))` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-492/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-492`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
