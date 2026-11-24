# task-1028 — Count slots at compile time

Return the number of elements in the array. Array lengths are const generic parameters. The byte size of storage is independent of the requested count.

```rust
pub fn slots<const N: usize>(_values: &[u16; N]) -> usize
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `slots(&[1, 2])` gives `2`.
- `slots(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1028/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1028`.

[Module reading](https://doc.rust-lang.org/reference/items/generics.html)
