# task-066 — Bundle labels · one reading

Each packet needs 2 item labels plus one address label. Count labels for value packets.

```rust
pub fn bundle_labels_2(value: u16) -> u32
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `bundle_labels_2(4)` gives `12`.
- `bundle_labels_2(13)` gives `39`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-066/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-066`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
