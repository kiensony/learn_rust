# task-2523 — ABI signed decode · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Advanced FFI lab. Define a local extern "C" callback with fixed-width unsigned arguments and result. Drive it through a typed C-ABI function pointer. Observe the absolute magnitude of the byte interpreted as signed two's complement. During review explain why substituting a Rust-ABI function pointer would change the interface.

Return Some((minimum, maximum)) or None for an empty trace. Use an extern "C" fn(u8) -> u32 callback. During review, point to the operation that enforces its contract.

```rust
pub fn abi_signed_decode_bounds(input: &[u8]) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `abi_signed_decode_bounds(&[])` gives `None`.
- `abi_signed_decode_bounds(&[4])` gives `Some((4u32, 4u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2523/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2523`.

[Module reading](https://doc.rust-lang.org/reference/items/external-blocks.html)
