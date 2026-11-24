# task-443 — Edit the caller’s label · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn remainder_after_first_mark_equals(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("72;12"); remainder_after_first_mark_equals(&mut data); data }` gives `""`.
- `{ let mut data = String::from("猫=12;note"); remainder_after_first_mark_equals(&mut data); data }` gives `"12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-443/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-443`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
