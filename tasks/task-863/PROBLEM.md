# task-863 — Publish a stable command type

Expose the command type at the crate root using the provided re-export. Implement the public predicate for Exit only. Keep the source module private and add a rustdoc usage example.

```rust
pub fn is_exit(command: Command) -> bool
```

Implement the public function in `src/lib.rs`.

## Public examples

- `is_exit(Command::Exit)` gives `true`.
- `is_exit(Command::Write(2))` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-863/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-863`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
