# task-599 — Return a borrowed field · leading marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn leading_marks_removed_hash<'a>(text: &'a str) -> &'a str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `leading_marks_removed_hash("outside[red#blue]ignored")` gives `"red#blue"`.
- `leading_marks_removed_hash("[猫###a]")` gives `"猫###a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-599/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-599`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
