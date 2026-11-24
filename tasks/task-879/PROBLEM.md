# task-879 — Access a module through its public interface

The private lookup table lives in wire. Call the crate-visible lookup method to get a code width without widening the visibility of the table.

```rust
pub fn code_size(code: u8) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `code_size(4)` gives `2`.
- `code_size(0)` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-879/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-879`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
