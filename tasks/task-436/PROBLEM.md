# task-436 — Edit the caller’s label · longest field with last tie (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the longest field with last tie. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn longest_field_with_last_tie_pipe(text: &mut String) -> ()
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `{ let mut data = String::from("first\n123"); longest_field_with_last_tie_pipe(&mut data); data }` gives `"123"`.
- `{ let mut data = String::from("hi|kit"); longest_field_with_last_tie_pipe(&mut data); data }` gives `"kit"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-436/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-436`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
