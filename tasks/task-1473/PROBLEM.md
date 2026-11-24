# task-1473 — Retry fixture · bounds

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 5xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return Some((minimum, maximum)) or None for an empty trace. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn retry_fixture_bounds(input: &str) -> Option<(u32, u32)>
```

Implement the public function in `src/lib.rs`.

## Public examples

- `retry_fixture_bounds("")` gives `None`.
- `retry_fixture_bounds("200")` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1473/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1473`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
