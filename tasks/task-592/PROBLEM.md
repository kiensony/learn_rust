# task-592 — Return a borrowed field · remainder after first mark (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn remainder_after_first_mark_equals<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `remainder_after_first_mark_equals("72;12")` gives `""`.
- `remainder_after_first_mark_equals("猫=12;note")` gives `"12"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-592/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-592`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
