# task-284 — Read a heading · tildes

Return everything before the first '~' or all text when absent.

```rust
pub fn read_a_heading_tilde(text: &str) -> String
```

Implement the public function in `src/lib.rs`.

## Public examples

- `read_a_heading_tilde("cat")` gives `"cat"`.
- `read_a_heading_tilde("~hi~")` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-284/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-284`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
