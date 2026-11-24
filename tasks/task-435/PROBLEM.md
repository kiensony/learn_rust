# task-435 — Edit the caller’s label · first nonempty field (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn first_nonempty_field_pipe(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("first\n123"); first_nonempty_field_pipe(&mut data); data }` gives `"123"`.
- `{ let mut data = String::from("header\n猫|123"); first_nonempty_field_pipe(&mut data); data }` gives `"猫"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-435/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-435`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
