# task-430 — Edit the caller’s label · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Replace the caller’s String with the selected field in place. Test the caller’s value after the function returns.

```rust
pub fn leading_marks_removed_pipe(text: &mut String) -> ()
```

Implement the public function in `src/lib.rs`.

## Public examples

- `{ let mut data = String::from("first\n123"); leading_marks_removed_pipe(&mut data); data }` gives `"123"`.
- `{ let mut data = String::from("header\n猫|123"); leading_marks_removed_pipe(&mut data); data }` gives `"猫|123"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-430/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-430`.

[Module reading](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
