# task-500 — Split a slice · leading marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Inspect the longer slice in the pair (first on a tie). Return the selected field’s byte length.

```rust
pub fn leading_marks_removed_hash(fields: (&str, &str)) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_hash(("outside[red#blue]ignored", "x"))` gives `8`.
- `leading_marks_removed_hash(("[猫###a]", "z#w"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-500/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-500`.

[Module reading](https://doc.rust-lang.org/book/ch04-03-slices.html)
