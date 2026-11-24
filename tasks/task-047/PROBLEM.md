# task-047 — Give an output separator one owner

Extract the divider into the supplied library function `pub fn divider() -> &'static str`. It must return exactly three equals signs. Use it for both borders of a one-line report containing the first argument. Empty input leaves an empty middle line.

From the repository directory, open `tasks/task-047/` and edit `src/main.rs`, `src/lib.rs`. The example commands below run inside that task directory.

```sh
cargo test --manifest-path tasks/task-047/Cargo.toml --test public
```

## Examples

Run the short example commands inside `tasks/task-047/`. Quoted output shows line breaks as `\n` and tabs as `\t`. A successful program exits with code zero and leaves standard error empty unless the exercise specifies a message there.

- Running `cargo run -- rain` produces standard output `"===\nrain\n===\n"`.
- Running `cargo run -- wind` produces standard output `"===\nwind\n===\n"`.

## Finish

Follow the submission step in `REVIEW.md` after the public examples pass. From the repository directory, run the complete check.

```sh
cargo test --manifest-path tasks/task-047/Cargo.toml
```
