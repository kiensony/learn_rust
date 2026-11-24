# task-088 — Two counters · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. One dial counts in groups of 5. Return both the complete groups and the remainder.

```rust
pub fn two_counters_5(issued: u16, returned: u16) -> (u16, u16)
```

Implement the public function in `src/lib.rs`.

## Public examples

- `two_counters_5(4, 0)` gives `(0, 4,)`.
- `two_counters_5(13, 1)` gives `(2, 2,)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-088/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-088`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
