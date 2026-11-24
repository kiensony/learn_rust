# task-419 — Edit the caller’s label · closing field (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn closing_field_slash(text: &mut String) -> ()
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `{ let mut data = String::from("path/1/12"); closing_field_slash(&mut data); data }` gives `"12"`.
- `{ let mut data = String::from("path/123"); closing_field_slash(&mut data); data }` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-419/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-419`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
