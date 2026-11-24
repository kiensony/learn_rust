# task-2507 — ABI unsigned widening · distinct

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the byte widened without sign extension. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return distinct observations in ascending order. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_unsigned_widening_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `abi_unsigned_widening_distinct(&[])` gives `vec![]`.
- `abi_unsigned_widening_distinct(&[4])` gives `vec![4u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2507/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2507`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
