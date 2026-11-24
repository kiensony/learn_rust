# task-446 — Edit the caller’s label · longest field with last tie (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn longest_field_with_last_tie_equals(text: &mut String) -> ()
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `{ let mut data = String::from("72;12"); longest_field_with_last_tie_equals(&mut data); data }` gives `"72"`.
- `{ let mut data = String::from("猫=12;note"); longest_field_with_last_tie_equals(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-446/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-446`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
