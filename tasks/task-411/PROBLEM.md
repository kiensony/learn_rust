# task-411 — Edit the caller’s label · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn closing_marks_removed_colon(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("hi:kit"); closing_marks_removed_colon(&mut data); data }` gives `"hi:kit"`.
- `{ let mut data = String::from(":é:a"); closing_marks_removed_colon(&mut data); data }` gives `":é:a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-411/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-411`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
