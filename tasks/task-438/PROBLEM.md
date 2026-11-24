# task-438 — Edit the caller’s label · leading field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn leading_field_equals(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("72;12"); leading_field_equals(&mut data); data }` gives `"72"`.
- `{ let mut data = String::from("猫=12;note"); leading_field_equals(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-438/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-438`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
