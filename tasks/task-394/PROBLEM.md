# task-394 — Inspect a borrowed label · prefix before final mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Accept a shared reference and return the byte count of the selected field. The caller keeps its String.

```rust
pub fn prefix_before_final_mark_equals(text: &String) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `prefix_before_final_mark_equals(&String::from("72;12"))` gives `0`.
- `prefix_before_final_mark_equals(&String::from("猫=12;note"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-394/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-394`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
