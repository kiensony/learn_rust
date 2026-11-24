# task-444 — Edit the caller’s label · prefix before final mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn prefix_before_final_mark_equals(text: &mut String) -> ()
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `{ let mut data = String::from("72;12"); prefix_before_final_mark_equals(&mut data); data }` gives `""`.
- `{ let mut data = String::from("猫=12;note"); prefix_before_final_mark_equals(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-444/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-444`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
