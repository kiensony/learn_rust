# task-417 — Edit the caller’s label · shortest field with first tie (:)

Treat ':' as the field separator. Select the shortest field with first tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn shortest_field_with_first_tie_colon(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("hi:kit"); shortest_field_with_first_tie_colon(&mut data); data }` gives `"hi"`.
- `{ let mut data = String::from(":é:a"); shortest_field_with_first_tie_colon(&mut data); data }` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-417/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-417`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
