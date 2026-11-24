# task-170 — Match a signal letter

Use a match expression to translate uppercase R to stop and G to go. Other characters yield wait. A match arm uses a pattern, => and its result. An underscore pattern accepts any remaining value.

```rust
pub fn instruction(signal: char) -> &'static str
```

Implement the public function in `src/lib.rs`.

## Public examples

- `instruction('R')` gives `"stop"`.
- `instruction('G')` gives `"go"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-170/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-170`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
