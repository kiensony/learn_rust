# task-1313 — Nibble fields · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Return Some((minimum, maximum)) or None for an empty trace. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_fields_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `nibble_fields_bounds("")` gives `None`.
- `nibble_fields_bounds("1")` gives `Some((0u32, 0u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1313/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1313`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
