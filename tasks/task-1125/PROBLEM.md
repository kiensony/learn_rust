# task-1125 — Read a trait object · marks removed at both ends (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the marks removed at both ends. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Read the field through a dynamically dispatched Source. Return its byte length for any concrete implementor. Explain which method is selected at runtime.

```rust
pub fn marks_removed_at_both_ends_pipe(source: &dyn Source) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `marks_removed_at_both_ends_pipe(&"first\n123")` gives `3`.
- `marks_removed_at_both_ends_pipe(&String::from("header\n猫|123"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1125/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1125`.

[Module reading](https://doc.rust-lang.org/reference/types/trait-object.html)
