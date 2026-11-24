# task-899 — Build a documented crate facade

Keep the implementation module private. Preserve the root re-export of Mode. Implement its wire values and document the public function with a runnable example using only crate-root imports.

```rust
pub fn wire_value(command: Mode) -> u8
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `wire_value(Mode::Listen)` gives `1`.
- `wire_value(Mode::Talk)` gives `2`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-899/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-899`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
