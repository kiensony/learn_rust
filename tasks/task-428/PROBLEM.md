# task-428 — Edit the caller’s label · leading field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn leading_field_pipe(text: &mut String) -> ()
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `{ let mut data = String::from("first\n123"); leading_field_pipe(&mut data); data }` gives `"123"`.
- `{ let mut data = String::from("header\n猫|123"); leading_field_pipe(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-428/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-428`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
