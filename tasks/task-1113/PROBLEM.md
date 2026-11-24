# task-1113 — Read a trait object · leading marks removed (/)

If the record begins with "path/", remove that one prefix first. Treat '/' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn leading_marks_removed_slash(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_slash(&"path/1/12")` gives `4`.
- `leading_marks_removed_slash(&String::from("path/123"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1113/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1113`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
