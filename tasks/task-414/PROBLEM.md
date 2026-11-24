# task-414 — Edit the caller’s label · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn prefix_before_final_mark_colon(text: &mut String) -> ()
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `{ let mut data = String::from("hi:kit"); prefix_before_final_mark_colon(&mut data); data }` gives `"hi"`.
- `{ let mut data = String::from(":é:a"); prefix_before_final_mark_colon(&mut data); data }` gives `":é"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-414/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-414`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
