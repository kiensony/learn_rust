# task-408 — Edit the caller’s label · leading field (:)

Treat ':' as the field separator. Select the leading field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn leading_field_colon(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("hi:kit"); leading_field_colon(&mut data); data }` gives `"hi"`.
- `{ let mut data = String::from(":é:a"); leading_field_colon(&mut data); data }` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-408/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-408`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
