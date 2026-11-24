# task-1343 — Packed booleans · bounds

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the number of one bits in each parsed byte.

Return Some((minimum, maximum)) or None for an empty trace. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn packed_booleans_bounds(input: &str) -> Option<(u32, u32)>
```

## Public examples

- `packed_booleans_bounds("")` gives `None`.
- `packed_booleans_bounds("1")` gives `Some((1u32, 1u32))`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1343/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1343`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
