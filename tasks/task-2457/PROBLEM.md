# task-2457 — A checked raw range

Labeled advanced unsafe lab. Complete a safe adapter that copies a subrange from a live byte slice. Return Some including for a valid empty range. Return None when offset + count overflows usize or the end exceeds the buffer length. Keep the raw operation inside one documented unsafe block. During review justify origin, initialized length, alignment, pointer arithmetic, and the duration of the resulting slice borrow. Run the tests with the separately pinned Miri setup in the course guide.

```rust
pub fn checked_copy(bytes: &[u8], offset: usize, count: usize) -> Option<Vec<u8>>
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `checked_copy(&[1,2,3], 1, 1)` gives `Some(vec![2])`.
- `checked_copy(&[1,2,3], 1, 2)` gives `Some(vec![2,3])`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2457/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2457`.

[Module reading](https://doc.rust-lang.org/reference/unsafety.html)
