# task-2529 — ABI signed decode · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the absolute magnitude of the byte interpreted as signed two's complement. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_signed_decode_longest_run(input: &[u8]) -> u64
```

## Public examples

- `abi_signed_decode_longest_run(&[])` gives `0`.
- `abi_signed_decode_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2529/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2529`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
