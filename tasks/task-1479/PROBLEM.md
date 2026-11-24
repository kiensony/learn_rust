# task-1479 — Retry fixture · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 5xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use an integration test outside src and the crate's public API. During review, point to the operation that enforces its contract.

```rust
pub fn retry_fixture_longest_run(input: &str) -> u64
```

## Public examples

- `retry_fixture_longest_run("")` gives `0`.
- `retry_fixture_longest_run("200")` gives `0`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1479/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1479`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
