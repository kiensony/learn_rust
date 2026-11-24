# task-618 — Read a record · leading marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the selected byte length plus the record’s revision. Extract the field calculation into a private method of Record.

```rust
pub fn leading_marks_removed_slash(record: &Record) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_slash(&Record { label: String::from("path/1/12"), revision: 0 })` gives `4`.
- `leading_marks_removed_slash(&Record { label: String::from("hi/kit"), revision: 2 })` gives `8`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-618/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-618`.

[Module reading](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
