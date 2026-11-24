# task-597 — Return a borrowed field · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return a slice of the input. Keep the explicit lifetime and borrow directly from the caller.

```rust
pub fn leading_field_hash<'a>(text: &'a str) -> &'a str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_field_hash("outside[red#blue]ignored")` gives `"red"`.
- `leading_field_hash("[猫###a]")` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-597/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-597`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
