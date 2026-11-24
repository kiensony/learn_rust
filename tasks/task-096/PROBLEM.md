# task-096 — Bundle labels · distance

Two readings mark the ends of an interval. Use their absolute difference as value, even when the readings arrive in reverse order. Each packet needs 8 item labels plus one address label. Count labels for value packets.

```rust
pub fn bundle_labels_8(start: u16, end: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `bundle_labels_8(4, 0)` gives `36`.
- `bundle_labels_8(13, 1)` gives `108`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-096/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-096`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
