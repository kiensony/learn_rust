# task-730 — Keep zero distinct from missing

Use standard u16 parsing. Return Some for every valid value, including zero, and None for a parse error. Leave whitespace untouched.

```rust
pub fn port(text: &str) -> Option<u16>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `port("80")` gives `Some(80)`.
- `port("0")` gives `Some(0)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-730/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-730`.

[Module reading](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
