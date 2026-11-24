# task-2696 — Lending windows · threshold crossing

Process consecutive borrowed chunks of up to 2 bytes. For each nonempty chunk observe the slice length. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use a generic associated type tied to the receiver borrow. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn lending_windows_threshold_crossing(input: &[u8], limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `lending_windows_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0)` gives `Some((0, 2))`.
- `lending_windows_threshold_crossing(&[], 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2696/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2696`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
