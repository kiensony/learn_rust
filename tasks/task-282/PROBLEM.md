# task-282 — Boundary trim · tildes

Remove all consecutive '~' characters at either end.

```rust
pub fn boundary_trim_tilde(text: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `boundary_trim_tilde("cat")` gives `"cat"`.
- `boundary_trim_tilde("~hi~")` gives `"hi"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-282/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-282`.

[Module reading](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
