# task-188 — Priority fee · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Return 0 for zero, 5 for a positive value through 7, and twice 5 otherwise.

```rust
pub fn priority_fee_5(issued: u16, returned: u16) -> u8
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `priority_fee_5(4, 0)` gives `5`.
- `priority_fee_5(13, 1)` gives `10`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-188/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-188`.

[Module reading](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
