# task-1461 — Client error fixture · trace

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 4xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the observations in input order, including duplicates. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn client_error_fixture_trace(input: &str) -> Vec<u32>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `client_error_fixture_trace("")` gives `vec![]`.
- `client_error_fixture_trace("200")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1461/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1461`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
