# task-451 — Edit the caller’s label · closing marks removed (#)

Read inside the first pair of square brackets. Without a complete pair, use an empty record. Treat '#' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn closing_marks_removed_hash(text: &mut String) -> ()
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `{ let mut data = String::from("outside[red#blue]ignored"); closing_marks_removed_hash(&mut data); data }` gives `"red#blue"`.
- `{ let mut data = String::from("[猫###a]"); closing_marks_removed_hash(&mut data); data }` gives `"猫###a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-451/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-451`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
