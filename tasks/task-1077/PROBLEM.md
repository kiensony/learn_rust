# task-1077 — Advance an associated iterator exactly once

Return the first item the iterator yields, or None. The result type must stay tied to the concrete iterator’s associated Item.

```rust
pub fn consume_first<I: Iterator>(mut items: I) -> Option<I::Item>
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `consume_first(vec![3,5].into_iter())` gives `Some(3)`.
- `consume_first(["α","β"].into_iter())` gives `Some("α")`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1077/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1077`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
