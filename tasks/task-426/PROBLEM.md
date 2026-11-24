# task-426 — Edit the caller’s label · longest field with last tie (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn longest_field_with_last_tie_slash(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("path/1/12"); longest_field_with_last_tie_slash(&mut data); data }` gives `"12"`.
- `{ let mut data = String::from("path/123"); longest_field_with_last_tie_slash(&mut data); data }` gives `"123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-426/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-426`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
