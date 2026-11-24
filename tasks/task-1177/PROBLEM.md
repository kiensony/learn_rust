# task-1177 — Return a borrowed key from a temporary query

Return the matching stored key reference. Its lifetime must depend only on the store. The search query may go out of scope immediately after the call.

```rust
pub fn stored_key<'a>(keys: &'a [&'a str], query: &str) -> Option<&'a str>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `stored_key(&["red","blue"], "blue")` gives `Some("blue")`.
- `stored_key(&["no"], "yes")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1177/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1177`.

[Module reading](https://doc.rust-lang.org/reference/trait-bounds.html)
