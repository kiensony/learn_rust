# task-939 — Call a supplied closure · prefix before final mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Call the visitor once on the selected field and return its result. The caller chooses the measurement.

```rust
pub fn prefix_before_final_mark_equals(text: &str, mut visit: impl FnMut(&str) -> usize) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `prefix_before_final_mark_equals("72;12", |s| s.chars().count() + 1)` gives `1`.
- `prefix_before_final_mark_equals("猫=12;note", |s| s.len() * 2)` gives `6`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-939/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-939`.

[Module reading](https://doc.rust-lang.org/book/ch13-01-closures.html)
