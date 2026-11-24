# task-588 — Return a borrowed field · closing field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the closing field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn closing_field_equals<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_field_equals("72;12")` gives `"72"`.
- `closing_field_equals("猫=12;note")` gives `"12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-588/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-588`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
