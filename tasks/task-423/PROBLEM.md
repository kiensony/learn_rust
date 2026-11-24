# task-423 — Edit the caller’s label · remainder after first mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the remainder after first mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn remainder_after_first_mark_slash(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("path/1/12"); remainder_after_first_mark_slash(&mut data); data }` gives `"12"`.
- `{ let mut data = String::from("path/123"); remainder_after_first_mark_slash(&mut data); data }` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-423/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-423`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
