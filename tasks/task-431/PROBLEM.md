# task-431 — Edit the caller’s label · closing marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn closing_marks_removed_pipe(text: &mut String) -> ()
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `{ let mut data = String::from("first\n123"); closing_marks_removed_pipe(&mut data); data }` gives `"123"`.
- `{ let mut data = String::from("header\n猫|123"); closing_marks_removed_pipe(&mut data); data }` gives `"猫|123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-431/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-431`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
