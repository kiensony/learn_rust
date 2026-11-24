# task-534 — Preserve valid UTF-8 on a byte budget

Borrow the longest prefix that fits in max_bytes and ends at a UTF-8 boundary. Preserve full scalars and handle a budget beyond the text length.

```rust
pub fn preview(text: &str, max_bytes: usize) -> &str
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `preview("rust", 2)` gives `"ru"`.
- `preview("葉ab", 2)` gives `""`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-534/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-534`.

[Module reading](https://doc.rust-lang.org/book/ch08-02-strings.html)
