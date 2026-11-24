# task-2537 — ABI endian swap · distinct

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the byte widened to u16 then byte-swapped. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return distinct observations in ascending order. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_endian_swap_distinct(input: &[u8]) -> Vec<u32>
```

## Public examples

- `abi_endian_swap_distinct(&[])` gives `vec![]`.
- `abi_endian_swap_distinct(&[4])` gives `vec![1024u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2537/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2537`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
