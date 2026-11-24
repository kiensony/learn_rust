# task-1462 — Client error fixture · total

Reproduce the starter failure. Trace one failing input through observation and reporting separately. Repair the first step whose result violates the contract and verify the counterexample.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 4xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the sum of all observations, widening each one before adding. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn client_error_fixture_total(input: &str) -> u64
```

## Public examples

- `client_error_fixture_total("")` gives `0`.
- `client_error_fixture_total("200")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1462/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1462`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
