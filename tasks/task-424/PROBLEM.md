# task-424 — Edit the caller’s label · prefix before final mark (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn prefix_before_final_mark_slash(text: &mut String) -> ()
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `{ let mut data = String::from("path/1/12"); prefix_before_final_mark_slash(&mut data); data }` gives `"1"`.
- `{ let mut data = String::from("path/123"); prefix_before_final_mark_slash(&mut data); data }` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-424/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-424`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
