# task-1257 — Command arguments · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Build a child command named report. The input is a list of UTF-8 tokens separated by ASCII whitespace, with no shell quoting. Inspect each argument's byte length. Do not spawn the child.

Return distinct observations in ascending order. Use Command::args and Command::get_args. During review, point to the operation that enforces its contract.

```rust
pub fn command_arguments_distinct(input: &str) -> Vec<u32>
```

## Public examples

- `command_arguments_distinct("")` gives `vec![]`.
- `command_arguments_distinct("only")` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1257/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1257`.

[Module reading](https://doc.rust-lang.org/std/process/)
