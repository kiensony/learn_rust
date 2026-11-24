# task-448 — Edit the caller’s label · leading field (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn leading_field_hash(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("outside[red#blue]ignored"); leading_field_hash(&mut data); data }` gives `"red"`.
- `{ let mut data = String::from("[猫###a]"); leading_field_hash(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-448/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-448`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
