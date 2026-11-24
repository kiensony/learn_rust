# task-413 — Edit the caller’s label · remainder after first mark (:)

Treat ':' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn remainder_after_first_mark_colon(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("hi:kit"); remainder_after_first_mark_colon(&mut data); data }` gives `"kit"`.
- `{ let mut data = String::from(":é:a"); remainder_after_first_mark_colon(&mut data); data }` gives `"é:a"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-413/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-413`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
