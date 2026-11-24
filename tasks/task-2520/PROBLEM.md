# task-2520 — ABI flags · reverse

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe zero for zero, one otherwise. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return the observations in reverse order. Retain duplicate observations. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_flags_reverse(input: &[u8]) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `abi_flags_reverse(&[])` gives `vec![]`.
- `abi_flags_reverse(&[4])` gives `vec![1u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2520/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2520`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
