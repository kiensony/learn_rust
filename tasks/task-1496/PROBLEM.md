# task-1496 — HTTP status fixture · threshold crossing

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe the hundred-class digit of each accepted code. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return the zero-based position and value of the first observation at least limit. Return None if no observation qualifies. The position refers to the derived sequence.

Use an integration test outside src and the crate's public API. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn http_status_fixture_threshold_crossing(input: &str, limit: u32) -> Option<(usize, u32)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `http_status_fixture_threshold_crossing("500 500 204 301 404", 0)` gives `Some((0, 5))`.
- `http_status_fixture_threshold_crossing("", 0)` gives `None`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1496/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1496`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
