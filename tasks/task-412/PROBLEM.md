# task-412 — Edit the caller’s label · marks removed at both ends (:)

Treat ':' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn marks_removed_at_both_ends_colon(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("hi:kit"); marks_removed_at_both_ends_colon(&mut data); data }` gives `"hi:kit"`.
- `{ let mut data = String::from(":é:a"); marks_removed_at_both_ends_colon(&mut data); data }` gives `"é:a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-412/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-412`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
