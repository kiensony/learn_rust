# task-1315 — Nibble fields · changes

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Split on commas, trim each field and parse it as a decimal u8. Ignore fields that cannot parse. Observe the high four bits of each parsed byte.

Collapse adjacent equal observations into one. Preserve the order and keep nonadjacent repetitions. Use str::parse and explicit handling of parse errors. During review, point to the operation that enforces its contract.

```rust
pub fn nibble_fields_changes(input: &str) -> Vec<u32>
```

## Public examples

- `nibble_fields_changes("")` gives `vec![]`.
- `nibble_fields_changes("1")` gives `vec![0u32]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1315/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1315`.

[Module reading](https://doc.rust-lang.org/std/str/trait.FromStr.html)
