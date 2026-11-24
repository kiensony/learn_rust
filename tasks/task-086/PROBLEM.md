# task-086 — Bundle labels · returns

Start with the issued count and subtract the returned count. Extra returns leave zero. Use the remaining count as value. Each packet needs 5 item labels plus one address label. Count labels for value packets.

```rust
pub fn bundle_labels_5(issued: u16, returned: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `bundle_labels_5(4, 0)` gives `24`.
- `bundle_labels_5(13, 1)` gives `72`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-086/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-086`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
