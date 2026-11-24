# task-2531 — ABI endian swap · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the byte widened to u16 then byte-swapped. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return the observations in input order, including duplicates. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_endian_swap_trace(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `abi_endian_swap_trace(&[])` gives `vec![]`.
- `abi_endian_swap_trace(&[4])` gives `vec![1024u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2531/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2531`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
