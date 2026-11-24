# task-076 — Bundle labels · two deliveries

Combine two deliveries, capping their sum at 65535. Use the combined count as value. Each packet needs 3 item labels plus one address label. Count labels for value packets.

```rust
pub fn bundle_labels_3(first: u16, second: u16) -> u32
```

Implement the public function in `src/lib.rs`.

## Public examples

- `bundle_labels_3(4, 0)` gives `16`.
- `bundle_labels_3(13, 3)` gives `64`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-076/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-076`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
