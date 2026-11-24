# task-583 — Borrow a report line without a temporary

Borrow the first line of a report. Use Rust lines semantics for LF and CRLF. An empty report produces the empty slice. Remove the allocation whose lifetime causes the diagnostic.

```rust
pub fn heading(report: &str) -> &str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `heading("red\nblue")` gives `"red"`.
- `heading("rain")` gives `"rain"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-583/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-583`.

[Module reading](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
