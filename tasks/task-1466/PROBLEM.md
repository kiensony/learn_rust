# task-1466 — Client error fixture · descending steps

Complete the missing part of the supplied function. Trace one input through the observation and reporting steps.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 4xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Count adjacent pairs for which the second observation is strictly smaller. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn client_error_fixture_descending_steps(input: &str) -> u64
```

Implement the public function in `src/lib.rs`.

## Public examples

- `client_error_fixture_descending_steps("")` gives `0`.
- `client_error_fixture_descending_steps("200")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1466/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1466`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
